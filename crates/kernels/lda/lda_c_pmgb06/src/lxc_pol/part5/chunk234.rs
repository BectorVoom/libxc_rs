//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 234/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk234<F: Float>(t123: F, t290: F, t317: F, t740: F, t107: F, t110: F, t117: F, t118: F, t122: F, t125: F, t199: F, t202: F, t295: F, t297: F, t305: F, t312: F, t315: F, t329: F, t342: F, t346: F, t389: F, t393: F, t399: F, t417: F, t423: F, t427: F, t550: F, t558: F, t566: F, t572: F, t61: F, t610: F, t616: F, t701: F, t709: F, t711: F, t717: F, t721: F, t725: F, t726: F, t734: F, t77: F, t81: F, t84: F) -> (F, F) {
    let t744 = F::cast_from(0.054045904796391424_f64) * t123 * t740 * t290 * t317;
    let t749 = F::new(3.0) * t329 * t77 * t342 + t346 * t389 + (t393 - F::cast_from(0.031505407223141116_f64) * t399 * t118 - t417 - t423 + t427 - F::cast_from(0.005388405304614574_f64) * t123 * t125 * t550 * t117) * t61 + (t558 - F::cast_from(0.0837628205355044_f64) * t399 * t199 - F::cast_from(0.0837628205355044_f64) * t84 * t566 + t572 - F::cast_from(0.011938374665504766_f64) * t122 * t202 * t610 - t616 + F::cast_from(0.42708890021612717_f64) * t107 * t110 * t701) * t295 + t709 - F::cast_from(0.01197423401025461_f64) * t297 * t711 - t717 - t721 + (t725 - F::cast_from(0.031835665774679375_f64) * t123 * t726 * t199 - F::cast_from(0.031835665774679375_f64) * t123 * t305 * t566 - t734 + F::cast_from(0.05332506774217938_f64) * t81 * t701) * t312 - t744 + F::cast_from(0.020267214298646783_f64) * t123 * t315 * t701 * t317;
    (t744, t749)
}
