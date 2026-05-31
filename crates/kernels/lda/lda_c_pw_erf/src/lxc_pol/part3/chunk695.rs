//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 695/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk695<F: Float>(t1556: F, t169: F, t1727: F, t1733: F, t1808: F, t2764: F, t2767: F, t2772: F, t2779: F, t2785: F, t2786: F, t279: F, t2791: F, t2793: F, t2799: F, t2801: F, t281: F, t2844: F, t2847: F, t2855: F, t2860: F, t2864: F, t2876: F, t296: F, t299: F, t301: F, t3196: F, t3199: F, t3203: F, t3366: F, t411: F, t4114: F, t4286: F, t453: F, t456: F) -> F {
    let t4288 = -F::cast_from(9.0_f64) * t2764 * t2767 - F::cast_from(0.16213771438917426_f64) * t2772 + F::cast_from(2.0_f64) * t453 * t2779 + t2785 + F::cast_from(18.0_f64) * t1808 * t2786 * t411 + t453 * t2791 + F::cast_from(9.0_f64) * t1733 * t2793 - F::cast_from(2.0_f64) * t1727 * t1556 - t453 * t2799 + F::cast_from(9.0_f64) * t1733 * t2801 + t2844 + t2847 - F::cast_from(0.01197423401025461_f64) * t281 * t2855 - F::cast_from(0.03592270203076383_f64) * t2860 - F::cast_from(0.03592270203076383_f64) * t2864 - t2876 + t3199 * t296 + F::cast_from(0.5945049527603057_f64) * t3203 + F::cast_from(0.020267214298646783_f64) * t169 * t299 * t3196 * t301 + t3366 * t456 + t4114 * t279 + t4286;
    t4288
}
