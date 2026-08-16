//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2314/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2314(t23384: f64, t28618: f64, t28671: f64, t82736: f64, t100326: f64, t100334: f64, t14651: f64, t1599: f64, t25479: f64, t25535: f64, t3186: f64, t3188: f64, t6687: f64, t7620: f64, t82809: f64, t89243: f64, t89421: f64, t89429: f64, t89431: f64, t89445: f64, t89501: f64) -> f64 {
    let t100378 = t23384 * t28618;
    let t100390 = t82736 * t28671;
    let t100396 = -0.18277045187202515961e-2_f64 * t100378 + 2.0_f64 * t14651 * t7620 + t89421 - t89429 - 0.36554090374405031923e-2_f64 * t89431 + 2.0_f64 * t3186 * t100326 * t3188 - 0.18277045187202515961e-2_f64 * t82809 - 0.16449340668482264365e-1_f64 * t6687 * t1599 * t25535 - 0.27415567780803773942e-2_f64 * t100390 - t89445 - 0.16449340668482264365e-1_f64 * t89243 * t25479 - 0.3289868133696452873e-1_f64 * t100334 * t89501;
    t100396
}
