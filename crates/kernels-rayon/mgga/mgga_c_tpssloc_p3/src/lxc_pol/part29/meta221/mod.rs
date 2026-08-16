//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1056;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta221(t1190: f64, t1751: f64, t1090: f64, t1735: f64, t3578: f64, t1216: f64, t1653: f64, t1222: f64, t1731: f64, t1744: f64, t1202: f64, t1743: f64, t225: f64, t4940: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4947, t4949, t4950, t4953, t4954, t4957, t4959, t4961) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1056(t1190, t1751, t1090, t1735, t3578, t1216, t1653, t1222, t1731, t1744, t1202, t1743);
        let t4964 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1057(t225, t4940);
    (t4947, t4949, t4950, t4953, t4954, t4957, t4959, t4961, t4964)
}
