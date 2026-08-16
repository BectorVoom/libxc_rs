//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 938/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk938<F: Float>(t3117: F, t3120: F, t3124: F, t3132: F, t3138: F, t3150: F, t3167: F, t3175: F, t4394: F, t2693: F, t2695: F, t887: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11272 = F::cast_from(192.98189186581325_f64) * t3117;
    let t11273 = F::cast_from(24.0_f64) * t3120;
    let t11274 = F::cast_from(24.0_f64) * t3124;
    let t11275 = F::cast_from(2069.0005882282467_f64) * t3132;
    let t11276 = F::cast_from(385.9637837316265_f64) * t3138;
    let t11277 = F::cast_from(4.0_f64) * t3150;
    let t11282 = F::cast_from(480.0_f64) * t3167;
    let t11286 = F::cast_from(240.0_f64) * t3175;
    let t11299 = F::cast_from(3.5089340384731225_f64) * t4394;
    let t11305 = t887 * t2693 * t2695;
    (t11272, t11273, t11274, t11275, t11276, t11277, t11282, t11286, t11299, t11305)
}
