//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1235/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1235(t2222: f64, t2242: f64, t2298: f64, t329: f64, t332: f64, t19631: f64, t20808: f64, t21011: f64, t21106: f64, t21762: f64, t21764: f64, t21768: f64, t21771: f64, t21775: f64, t21777: f64, t21781: f64, t21785: f64, t2220: f64, t2353: f64, t2383: f64, t2387: f64, t2408: f64, t2409: f64, t2410: f64, t326: f64, t335: f64, t338: f64, t353: f64, t376: f64, t6159: f64, t6164: f64, t822: f64, t826: f64, t827: f64, t833: f64) -> f64 {
    let t21788 = t2242 * t2222;
    let t21807 = t329 * t332 * t2298;
    let t21813 = t2387 * t6159 * t6164 / 16.0_f64 + 7.0_f64 / 36.0_f64 * t21762 - t822 * t21764 * t21768 / 16.0_f64 + 7.0_f64 / 72.0_f64 * t21771 + 7.0_f64 / 36.0_f64 * t21775 + 7.0_f64 / 12.0_f64 * t21777 + 35.0_f64 / 12.0_f64 * t21781 + t827 * t21785 / 24.0_f64 + 35.0_f64 / 72.0_f64 * t21788 + t326 * t20808 * t2383 * t833 / 32.0_f64 + t326 * t21106 * t826 * t833 / 96.0_f64 + t2408 * t2409 * t19631 * t2410 / 4.0_f64 - t335 * t338 * t2220 * t2353 / 16.0_f64 + 5.0_f64 / 4.0_f64 * t21807 * t338 * t353 * t376 * t21011;
    t21813
}
