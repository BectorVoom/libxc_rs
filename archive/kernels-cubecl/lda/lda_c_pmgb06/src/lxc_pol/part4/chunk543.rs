//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 543/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk543<F: Float>(t113: F, t1798: F, t301: F, t413: F, t794: F, t297: F, t707: F, t909: F, t1750: F, t1753: F, t1760: F, t1765: F, t2168: F, t2172: F, t2176: F, t2180: F, t2181: F, t2258: F, t295: F, t342: F, t346: F, t384: F, t73: F, t790: F) -> (F, F, F, F, F) {
    let t2262 = t1798 * t113 * t301;
    let t2266 = t794 * t413 * t301;
    let t2267 = t297 * t2266;
    let t2269 = t707 * t909;
    let t2273 = t2168 * t295 - F::cast_from(0.054045904796391424_f64) * t2172 - F::cast_from(0.0002905674151788692_f64) * t2176 + t346 * t790 * t384 + F::cast_from(6.0_f64) * t2180 * t2181 * t342 + t346 * t2258 * t73 - F::cast_from(0.01197423401025461_f64) * t297 * t2262 - F::cast_from(0.01197423401025461_f64) * t2267 + F::cast_from(0.019957056683757683_f64) * t2269 + F::cast_from(0.019957056683757683_f64) * t1750 + t1753 - F::cast_from(0.01197423401025461_f64) * t1760 - t1765;
    (t2262, t2266, t2267, t2269, t2273)
}
