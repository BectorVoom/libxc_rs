//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 535/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk535<F: Float>(t2695: F, t77: F, t73: F, t783: F, t2311: F, t790: F, t1155: F, t1205: F, t123: F, t1233: F, t1316: F, t199: F, t2283: F, t2293: F, t2302: F, t2308: F, t2407: F, t2415: F, t2422: F, t2432: F, t2449: F, t2667: F, t2692: F, t2733: F, t295: F, t297: F, t305: F, t312: F, t315: F, t317: F, t329: F, t346: F, t61: F, t787: F, t81: F, t868: F, t912: F) -> (F, F, F) {
    let t2738 = t77 * t2695;
    let t2741 = t73 * t783;
    let t2744 = t790 * t2311;
    let t2747 = F::cast_from(0.020267214298646783_f64) * t123 * t315 * t2407 * t317 + (-t1155 + F::cast_from(0.10611888591559791_f64) * t2283 + F::cast_from(0.10611888591559791_f64) * t2293 - F::cast_from(0.031835665774679375_f64) * t123 * t2415 * t199 - F::cast_from(0.06367133154935875_f64) * t123 * t912 * t868 - F::cast_from(0.031835665774679375_f64) * t123 * t305 * t2422 + t1205 - F::cast_from(0.2133002709687175_f64) * t2302 + F::cast_from(0.05332506774217938_f64) * t81 * t2407) * t312 - F::cast_from(0.01197423401025461_f64) * t297 * t2432 + F::cast_from(3.0_f64) * t329 * t2449 + t2667 * t295 + t2692 * t61 + t346 * t2733 * t73 + t346 * t790 * t787 + F::cast_from(6.0_f64) * t1233 * t2738 - t346 * t2308 * t2741 + F::cast_from(6.0_f64) * t1316 * t2744;
    (t2738, t2741, t2747)
}
