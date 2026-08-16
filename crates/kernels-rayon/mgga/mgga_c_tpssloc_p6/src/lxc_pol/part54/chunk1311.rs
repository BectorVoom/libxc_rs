//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1311/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1311(t30714: f64, t4240: f64, t4250: f64, t4191: f64, t23270: f64, t25038: f64, t30622: f64, t4255: f64, t22986: f64, t4119: f64, t32814: f64, t81651: f64, t82074: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118608 = t30714 * t4240;
    let t118610 = t30714 * t4250;
    let t118612 = t30714 * t4191;
    let t118626 = 0.9869604401089358619e-1_f64 * t25038 * t23270 * t30622 * t4255;
    let t118630 = 0.3289868133696452873e-1_f64 * t22986 * t23270 * t30622 * t4119;
    let t118632 = t81651 * t82074 * t32814;
    (t118608, t118610, t118612, t118626, t118630, t118632)
}
