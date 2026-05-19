//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 751/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk751<F: Float>(t3011: F, t890: F, t101: F, t3014: F, t15283: F, t102: F, t15274: F, t15278: F, t15279: F, t15285: F, t15312: F, t15317: F, t15319: F, t15331: F, t15336: F, t15339: F, t15345: F, t3006: F, t3012: F, t69: F, t857: F, t863: F, t884: F, t889: F, t89: F, t918: F, t969: F) -> F {
    let t15349 = F::new(1.0) / t3011 / t890;
    let t15352 = F::new(1.0) / t3014 / t101;
    let t15353 = t15349 * t15283 * t15352;
    let t15363 = -F::cast_from(0.51947267698127589897e2_f64) * t889 * t3012 * t3006 * t15274 + F::cast_from(0.35089340384731224426e1_f64) * t889 * t15278 * t15279 - F::cast_from(0.35089340384731224426e1_f64) * t889 * t15285 - F::cast_from(0.58482233974552040708e0_f64) * t889 * t15312 - F::cast_from(0.96490945932906628932e2_f64) * t15317 * t15319 + F::new(1.0) * t863 * t15331 + F::cast_from(0.51725014705706168417e3_f64) * t15336 * t15339 + F::cast_from(0.1038945353962551798e3_f64) * t889 * t15345 - F::cast_from(0.1025389702100779493e4_f64) * t889 * t15353 + F::cast_from(0.34451131037037037036e-2_f64) * t857 * t969 * t89 - F::cast_from(0.56969282336565386482e-3_f64) * t884 * t69 * t918 * t102;
    t15363
}
