//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1402/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1402(t23366: f64, t23384: f64, t23582: f64, t23333: f64, t82431: f64, t10167: f64, t10170: f64, t10182: f64, t11010: f64, t11085: f64, t1956: f64, t23317: f64, t23323: f64, t23327: f64, t23329: f64, t23346: f64, t23402: f64, t23581: f64, t23722: f64, t23725: f64, t3026: f64, t3169: f64, t43619: f64, t6687: f64, t6700: f64, t6771: f64, t6776: f64, t6816: f64, t82441: f64, t884: f64) -> f64 {
    let t83316 = t23384 * t23366;
    let t83318 = t23384 * t23582;
    let t83329 = t82431 * t23333;
    let t83341 = -t6771 * t11085 - 3.0_f64 * t3026 * t23722 + 6.0_f64 * t11010 * t6776 - t43619 * t1956 - 0.16449340668482264365e-1_f64 * t83316 + 0.54831135561607547883e-2_f64 * t83318 + 6.0_f64 * t6771 * t10182 - 3.0_f64 * t10170 * t6816 + 12.0_f64 * t3169 * t23725 - 0.16449340668482264365e-1_f64 * t6687 * t23581 * t23402 - 0.54831135561607547883e-2_f64 * t83329 + 0.16449340668482264365e-1_f64 * t23327 * t23329 * t82441 * t884 - 6.0_f64 * t6771 * t10167 + 0.24125699647107321069e0_f64 * t23323 * t6700 + 0.65797362673929057459e-1_f64 * t23346 * t23317;
    t83341
}
