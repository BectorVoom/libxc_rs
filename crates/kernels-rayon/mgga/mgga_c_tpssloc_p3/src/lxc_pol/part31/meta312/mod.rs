//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta312 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta312(t11647: f64, t485: f64, t3576: f64, t3604: f64, t3585: f64, t820: f64, t10401: f64, t3575: f64, t3610: f64, t3624: f64, t3521: f64, t1190: f64, t3030: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11649, t11665, t11668, t11678, t11692, t11697, t11707) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1201(t11647, t485, t3576, t3604, t3585, t820, t10401, t3575, t3610, t3624, t3521, t1190, t3030);
    (t11649, t11665, t11668, t11678, t11692, t11697, t11707)
}
