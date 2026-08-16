//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1170/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1170(t23197: f64, t6547: f64, t23222: f64, t23237: f64, t6552: f64, t23257: f64, t6562: f64, t794: f64, t10109: f64, t225: f64, t10111: f64, t1880: f64, t6553: f64) -> (f64, f64, f64, f64) {
    let t82230 = t6547 * t23197;
    let t82233 = t6552 * t23237 * t23222;
    let t82236 = t6562 * t794 * t23257;
    let t82252 = t225 * t10109;
    let t82255 = t1880 * t6553 * t82252 * t10111;
    (t82230, t82233, t82236, t82255)
}
