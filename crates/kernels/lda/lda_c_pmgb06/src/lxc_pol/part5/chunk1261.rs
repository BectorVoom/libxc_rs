//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1261/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1261<F: Float>(t1322: F, t7353: F, t123: F, t1316: F, t1324: F, t14570: F, t14571: F, t14575: F, t14601: F, t18453: F, t18481: F, t18939: F, t2180: F, t21827: F, t22063: F, t22074: F, t2209: F, t22097: F, t22113: F, t2255: F, t2258: F, t24: F, t2730: F, t2733: F, t295: F, t315: F, t317: F, t342: F, t346: F, t5721: F, t5934: F, t6009: F, t6021: F, t7102: F, t769: F, t787: F, t7882: F, t790: F) -> F {
    let t22120 = t7353 * t1322;
    let t22123 = F::new(4.0) * t24 * t18939 * t6009 + F::new(3.0) * t1316 * t790 * t2730 * t342 + F::new(2.0) * t346 * t14601 * t7882 + F::new(9.0) * t1316 * t2258 * t7102 - F::cast_from(5.4655730795145296e-05_f64) * t18453 + F::cast_from(0.020267214298646783_f64) * t123 * t315 * t21827 * t317 + t14570 + F::cast_from(0.17961351015381913_f64) * t14571 + F::cast_from(0.0004919015771563077_f64) * t14575 + F::new(9.0) * t1316 * t790 * t787 * t2209 + F::new(18.0) * t2180 * t790 * t18481 - F::new(2.0) * t346 * t6021 * t5934 + t346 * t2258 * t2730 + F::new(9.0) * t1316 * t790 * t2255 * t769 + (t22063 + t22074 + t22097 + t22113) * t295 + F::new(9.0) * t1316 * t2733 * t5721 - t346 * t22120 * t1324;
    t22123
}
