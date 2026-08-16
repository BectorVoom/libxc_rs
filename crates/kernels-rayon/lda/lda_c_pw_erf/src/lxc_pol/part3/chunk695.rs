//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 695/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk695(t1556: f64, t169: f64, t1727: f64, t1733: f64, t1808: f64, t2764: f64, t2767: f64, t2772: f64, t2779: f64, t2785: f64, t2786: f64, t279: f64, t2791: f64, t2793: f64, t2799: f64, t2801: f64, t281: f64, t2844: f64, t2847: f64, t2855: f64, t2860: f64, t2864: f64, t2876: f64, t296: f64, t299: f64, t301: f64, t3196: f64, t3199: f64, t3203: f64, t3366: f64, t411: f64, t4114: f64, t4286: f64, t453: f64, t456: f64) -> f64 {
    let t4288 = -9.0_f64 * t2764 * t2767 - 0.16213771438917426_f64 * t2772 + 2.0_f64 * t453 * t2779 + t2785 + 18.0_f64 * t1808 * t2786 * t411 + t453 * t2791 + 9.0_f64 * t1733 * t2793 - 2.0_f64 * t1727 * t1556 - t453 * t2799 + 9.0_f64 * t1733 * t2801 + t2844 + t2847 - 0.01197423401025461_f64 * t281 * t2855 - 0.03592270203076383_f64 * t2860 - 0.03592270203076383_f64 * t2864 - t2876 + t3199 * t296 + 0.5945049527603057_f64 * t3203 + 0.020267214298646783_f64 * t169 * t299 * t3196 * t301 + t3366 * t456 + t4114 * t279 + t4286;
    t4288
}
