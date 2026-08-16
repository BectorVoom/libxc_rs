//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta311(t3242: f64, t460: f64, t3247: f64, t1176: f64, t134: f64, t1184: f64, t1239: f64, t68: f64, t1203: f64, t3540: f64, t2393: f64, t374: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11570, t11583, t11588, t11589, t11606, t11644, t11647) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1200(t3242, t460, t3247, t1176, t134, t1184, t1239, t68, t1203, t3540, t2393, t374, t486);
    (t11570, t11583, t11588, t11589, t11606, t11644, t11647)
}
