//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 844/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk844<F: Float>(t1752: F, t1753: F, t279: F, t2824: F, t3117: F, t3120: F, t3124: F, t3132: F, t3138: F, t3150: F, t3167: F, t3175: F, t4394: F, t2693: F, t2695: F, t887: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11266 = 16.521134411652657 * t1752 * t1753 * t2824 * t279;
    let t11272 = 192.98189186581325 * t3117;
    let t11273 = 24.0 * t3120;
    let t11274 = 24.0 * t3124;
    let t11275 = 2069.0005882282467 * t3132;
    let t11276 = 385.9637837316265 * t3138;
    let t11277 = 4.0 * t3150;
    let t11282 = 480.0 * t3167;
    let t11286 = 240.0 * t3175;
    let t11299 = 3.5089340384731225 * t4394;
    let t11305 = t887 * t2693 * t2695;
    (t11266, t11272, t11273, t11274, t11275, t11276, t11277, t11282, t11286, t11299, t11305)
}
