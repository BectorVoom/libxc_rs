//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1403/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1403(t23323: f64, t6683: f64, t23357: f64, t6680: f64, t23494: f64, t381: f64, t23384: f64, t23403: f64, t23589: f64, t6695: f64, t82632: f64, t10170: f64, t1956: f64, t23327: f64, t23337: f64, t23341: f64, t23346: f64, t23372: f64, t23378: f64, t23725: f64, t3026: f64, t3169: f64, t3207: f64, t43599: f64, t6691: f64, t6707: f64, t6776: f64, t82382: f64, t82402: f64) -> f64 {
    let t83342 = t23323 * t6683;
    let t83344 = t6680 * t23357;
    let t83352 = t23494 * t381;
    let t83358 = t23384 * t23403;
    let t83364 = t23384 * t23589;
    let t83368 = t82632 * t6695;
    let t83376 = 0.80418998823691070229e-1_f64 * t83342 + 0.14621636149762012769e-1_f64 * t83344 + 12.0_f64 * t3026 * t23725 - 18.0_f64 * t3169 * t23341 + 6.0_f64 * t3026 * t23378 - 0.82246703342411321826e-2_f64 * t23327 * t83352 * t6691 + 0.43864908449286038307e-1_f64 * t82402 * t23337 - 0.54831135561607547883e-2_f64 * t83358 + 0.43864908449286038307e-1_f64 * t23346 * t23403 - 0.13159472534785811492e0_f64 * t23346 * t23589 + 0.16449340668482264365e-1_f64 * t83364 - 0.24125699647107321069e0_f64 * t82382 * t6707 + 0.54831135561607547884e-2_f64 * t83368 - 3.0_f64 * t23372 * t3207 + 6.0_f64 * t10170 * t6776 - 3.0_f64 * t43599 * t1956;
    t83376
}
