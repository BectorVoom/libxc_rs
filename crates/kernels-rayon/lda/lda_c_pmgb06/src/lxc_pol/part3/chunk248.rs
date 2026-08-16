//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 248/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk248(t123: f64, t290: f64, t317: f64, t740: f64, t107: f64, t110: f64, t117: f64, t118: f64, t122: f64, t125: f64, t199: f64, t202: f64, t295: f64, t297: f64, t305: f64, t312: f64, t315: f64, t329: f64, t342: f64, t346: f64, t389: f64, t393: f64, t399: f64, t417: f64, t423: f64, t427: f64, t550: f64, t558: f64, t566: f64, t572: f64, t61: f64, t610: f64, t616: f64, t701: f64, t709: f64, t711: f64, t717: f64, t721: f64, t725: f64, t726: f64, t734: f64, t77: f64, t81: f64, t84: f64) -> (f64, f64) {
    let t744 = 0.054045904796391424_f64 * t123 * t740 * t290 * t317;
    let t749 = 3.0_f64 * t329 * t77 * t342 + t346 * t389 + (t393 - 0.031505407223141116_f64 * t399 * t118 - t417 - t423 + t427 - 0.005388405304614574_f64 * t123 * t125 * t550 * t117) * t61 + (t558 - 0.0837628205355044_f64 * t399 * t199 - 0.0837628205355044_f64 * t84 * t566 + t572 - 0.011938374665504766_f64 * t122 * t202 * t610 - t616 + 0.42708890021612717_f64 * t107 * t110 * t701) * t295 + t709 - 0.01197423401025461_f64 * t297 * t711 - t717 - t721 + (t725 - 0.031835665774679375_f64 * t123 * t726 * t199 - 0.031835665774679375_f64 * t123 * t305 * t566 - t734 + 0.05332506774217938_f64 * t81 * t701) * t312 - t744 + 0.020267214298646783_f64 * t123 * t315 * t701 * t317;
    (t744, t749)
}
