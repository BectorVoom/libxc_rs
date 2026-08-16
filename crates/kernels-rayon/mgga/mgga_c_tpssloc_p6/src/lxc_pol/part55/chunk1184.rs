//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1184/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1184(t118582: f64, t118615: f64, t23270: f64, t25038: f64, t30622: f64, t4255: f64, t22986: f64, t4119: f64, t32814: f64, t81651: f64, t82074: f64, t118510: f64, t118518: f64, t118523: f64, t118526: f64, t1527: f64, t218: f64, t25168: f64, t25169: f64, t25183: f64, t25199: f64, t25200: f64, t259: f64, t2718: f64, t30651: f64, t30728: f64, t30729: f64, t30741: f64, t32852: f64, t4147: f64, t4273: f64, t6627: f64, t855: f64, t865: f64, t866: f64) -> (f64, f64) {
    let t118616 = t118582 + t118615;
    let t118626 = 0.9869604401089358619e-1_f64 * t25038 * t23270 * t30622 * t4255;
    let t118630 = 0.3289868133696452873e-1_f64 * t22986 * t23270 * t30622 * t4119;
    let t118632 = t81651 * t82074 * t32814;
    let t118633 = 0.16449340668482264365e-1_f64 * t118632;
    let t118634 = 2.0_f64 * t1527 * t2718 * t30728 * t855 + 2.0_f64 * t2718 * t32852 * t855 * t865 + t118616 * t218 * t259 - 12.0_f64 * t25168 * t25169 * t25183 - 12.0_f64 * t25168 * t25169 * t25199 - t118510 * t866 + 4.0_f64 * t25200 * t6627 - 6.0_f64 * t30651 * t4147 - t30729 * t4147 + 2.0_f64 * t30741 * t4273 - t118518 - t118523 - t118526 - t118626 + t118630 - t118633;
    (t118616, t118634)
}
