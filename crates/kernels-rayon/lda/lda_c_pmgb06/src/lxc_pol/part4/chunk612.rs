//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 612/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk612(t2695: f64, t77: f64, t73: f64, t783: f64, t2311: f64, t790: f64, t1155: f64, t1205: f64, t123: f64, t1233: f64, t1316: f64, t199: f64, t2283: f64, t2293: f64, t2302: f64, t2308: f64, t2407: f64, t2415: f64, t2422: f64, t2432: f64, t2449: f64, t2667: f64, t2692: f64, t2733: f64, t295: f64, t297: f64, t305: f64, t312: f64, t315: f64, t317: f64, t329: f64, t346: f64, t61: f64, t787: f64, t81: f64, t868: f64, t912: f64) -> (f64, f64, f64, f64) {
    let t2738 = t77 * t2695;
    let t2741 = t73 * t783;
    let t2744 = t790 * t2311;
    let t2747 = 0.020267214298646783_f64 * t123 * t315 * t2407 * t317 + (-t1155 + 0.10611888591559791_f64 * t2283 + 0.10611888591559791_f64 * t2293 - 0.031835665774679375_f64 * t123 * t2415 * t199 - 0.06367133154935875_f64 * t123 * t912 * t868 - 0.031835665774679375_f64 * t123 * t305 * t2422 + t1205 - 0.2133002709687175_f64 * t2302 + 0.05332506774217938_f64 * t81 * t2407) * t312 - 0.01197423401025461_f64 * t297 * t2432 + 3.0_f64 * t329 * t2449 + t2667 * t295 + t2692 * t61 + t346 * t2733 * t73 + t346 * t790 * t787 + 6.0_f64 * t1233 * t2738 - t346 * t2308 * t2741 + 6.0_f64 * t1316 * t2744;
    (t2738, t2741, t2744, t2747)
}
