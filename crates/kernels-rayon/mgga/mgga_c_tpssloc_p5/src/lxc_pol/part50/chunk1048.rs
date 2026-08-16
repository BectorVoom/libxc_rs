//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1048/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1048(t1055: f64, t30899: f64, t23394: f64, t6775: f64, t6704: f64, t23365: f64, t8380: f64, t23384: f64, t8381: f64, t225: f64, t8392: f64, t1052: f64, t1066: f64, t1956: f64, t23346: f64, t23369: f64, t23372: f64, t3026: f64, t30855: f64, t30858: f64, t30862: f64, t30869: f64, t3169: f64, t6687: f64, t8397: f64, t8407: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30900 = t1055 * t30899;
    let t30904 = t23394 * t6775;
    let t30905 = t6704 * t30904;
    let t30908 = t23365 * t8380;
    let t30912 = 0.54831135561607547883e-2_f64 * t23384 * t8381;
    let t30915 = t8392 * t225;
    let t30919 = -0.16449340668482264365e-1_f64 * t6687 * t30855 - 0.16449340668482264365e-1_f64 * t6687 * t30858 + 0.16449340668482264365e-1_f64 * t6687 * t30862 + 2.0_f64 * t3026 * t8397 - t3169 * t8407 - 0.16449340668482264365e-1_f64 * t6687 * t30869 - t1052 * t30900 + 0.43864908449286038307e-1_f64 * t23346 * t8381 + 0.3289868133696452873e-1_f64 * t6687 * t30905 - 0.16449340668482264365e-1_f64 * t6687 * t30908 - t30912 - 2.0_f64 * t23372 * t1956 - t30915 * t1066 - 2.0_f64 * t23369 * t1956;
    (t30900, t30904, t30905, t30908, t30912, t30915, t30919)
}
