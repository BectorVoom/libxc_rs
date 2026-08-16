//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 526/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk526(t113: f64, t1798: f64, t301: f64, t413: f64, t794: f64, t297: f64, t707: f64, t909: f64, t1750: f64, t1753: f64, t1760: f64, t1765: f64, t2168: f64, t2172: f64, t2176: f64, t2180: f64, t2181: f64, t2258: f64, t295: f64, t342: f64, t346: f64, t384: f64, t73: f64, t790: f64) -> (f64, f64, f64) {
    let t2262 = t1798 * t113 * t301;
    let t2266 = t794 * t413 * t301;
    let t2267 = t297 * t2266;
    let t2269 = t707 * t909;
    let t2273 = t2168 * t295 - 0.054045904796391424_f64 * t2172 - 0.0002905674151788692_f64 * t2176 + t346 * t790 * t384 + 6.0_f64 * t2180 * t2181 * t342 + t346 * t2258 * t73 - 0.01197423401025461_f64 * t297 * t2262 - 0.01197423401025461_f64 * t2267 + 0.019957056683757683_f64 * t2269 + 0.019957056683757683_f64 * t1750 + t1753 - 0.01197423401025461_f64 * t1760 - t1765;
    (t2262, t2266, t2273)
}
