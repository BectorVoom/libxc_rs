//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta102 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk617;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk618;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta102(t138: f64, t681: f64, t125: f64, t2412: f64, t702: f64, t118: f64, t142: f64, t2393: f64) -> (f64, f64, f64, f64, f64) {
        let (t2419, t2420, t2421, t2423) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk617(t138, t681, t125, t2412, t702);
        let t2426 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk618(t118, t142, t2393);
    (t2419, t2420, t2421, t2423, t2426)
}
