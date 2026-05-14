//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 871/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk871<F: Float>(t3132: F, t3138: F, t3150: F, t3167: F, t3175: F, t4394: F, t2705: F, t2740: F, t2743: F, t4385: F, t7324: F, t7325: F, t7326: F, t7327: F, t7328: F, t7329: F, t7330: F, t7332: F, t8097: F, t8098: F, t8099: F, t8101: F, t8102: F, t8103: F) -> (F, F, F, F, F, F) {
    let t11275 = 2069.0005882282467 * t3132;
    let t11276 = 385.9637837316265 * t3138;
    let t11277 = 4.0 * t3150;
    let t11282 = 480.0 * t3167;
    let t11286 = 240.0 * t3175;
    let t11299 = 3.5089340384731225 * t4394;
    let t11302 = 3.0 * t4385 - t8097 + t8098 + t8099 + 0.09759222794503372 * t2705 - t8101 - t8102 + t7324 - t7325 - t7326 - t11299 - t8103 - 5.263401057709683 * t2740 + t7327 - 3.5089340384731225 * t2743 + t7328 - t7329 + t7330 + t7332;
    (t11275, t11276, t11277, t11282, t11286, t11302)
}
