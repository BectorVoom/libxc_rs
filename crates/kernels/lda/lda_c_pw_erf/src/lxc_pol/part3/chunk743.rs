//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 743/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk743<F: Float>(t128: F, t5548: F, t10: F, t3268: F, t3270: F, t3274: F, t3276: F, t3213: F, t3217: F, t3220: F, t3228: F, t3231: F, t3260: F, t3264: F, t426: F, t5502: F, t5505: F, t5507: F, t5511: F, t5513: F, t5517: F, t5520: F, t5523: F) -> (F, F, F, F, F, F, F) {
    let t5549 = t128 * t5548;
    let t5550 = t10 * t5549;
    let t5560 = 0.6495611111111111 * t3268;
    let t5561 = 0.48717083333333333 * t3270;
    let t5562 = 0.9743416666666667 * t3274;
    let t5563 = 1.2991222222222223 * t3276;
    let t5564 = -0.97936 * t5502 - t5505 - 2.0 / 9.0 * t5507 + t5511 - 0.48968 * t5513 + t5517 + t5520 + t5523 - t426 * t5550 / 2.0 - 2.93808 * t3213 - 1.95872 * t3217 - t3220 / 2.0 - 4.0 / 9.0 * t3228 + t3231 / 6.0 - 0.97936 * t3260 + 0.73452 * t3264 + t5560 + t5561 - t5562 - t5563;
    (t5549, t5550, t5560, t5561, t5562, t5563, t5564)
}
