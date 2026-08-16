//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta306(t10294: f64, t268: f64, t271: f64, t6546: f64, t2394: f64, t885: f64) -> (f64, f64, f64, f64) {
        let (t10542, t10544, t10545, t10556) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1329(t10294, t268, t271, t6546, t2394, t885);
    (t10542, t10544, t10545, t10556)
}
