//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk839;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta183(t491: f64, t4940: f64, t1235: f64, t1720: f64, t1721: f64, t225: f64, t1190: f64, t1751: f64, t1090: f64, t1735: f64, t3578: f64, t1216: f64, t1653: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4941, t4943, t4945, t4947, t4949, t4950, t4953) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk839(t491, t4940, t1235, t1720, t1721, t225, t1190, t1751, t1090, t1735, t3578, t1216, t1653);
    (t4941, t4943, t4945, t4947, t4949, t4950, t4953)
}
