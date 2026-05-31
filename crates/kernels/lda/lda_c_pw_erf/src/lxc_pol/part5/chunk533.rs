//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 533/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk533<F: Float>(t142: F, t770: F, t1554: F, t1101: F, t1108: F, t1146: F, t1148: F, t125: F, t145: F, t169: F, t1729: F, t2208: F, t2211: F, t2222: F, t2229: F, t2233: F, t2357: F, t2364: F, t2375: F, t242: F, t2589: F, t2592: F, t2595: F, t2645: F, t2647: F, t2673: F, t279: F, t281: F, t296: F, t299: F, t301: F, t405: F, t456: F, t777: F) -> (F, F, F) {
    let t2675 = t142 * t770;
    let t2676 = t1554 * t2675;
    let t2680 = F::cast_from(0.020267214298646783_f64) * t169 * t299 * t2357 * t301 + (-t1101 + F::cast_from(0.10611888591559791_f64) * t2222 + t1108 - F::cast_from(0.031835665774679375_f64) * t169 * t2364 * t242 - F::cast_from(0.06367133154935875_f64) * t2229 - t1146 + t1148 - F::cast_from(0.2133002709687175_f64) * t2233 + F::cast_from(0.05332506774217938_f64) * t145 * t2357) * t296 - F::cast_from(0.01197423401025461_f64) * t281 * t2375 + t2589 * t279 + t777 * t2592 + F::cast_from(6.0_f64) * t1729 * t2595 + t2645 * t456 + F::cast_from(3.0_f64) * t405 * t2647 + t2673 * t125 - t777 * t2676 + F::cast_from(6.0_f64) * t2211 * t2208;
    (t2675, t2676, t2680)
}
