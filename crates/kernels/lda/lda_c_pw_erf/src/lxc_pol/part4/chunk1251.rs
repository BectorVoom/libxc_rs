//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1251/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1251<F: Float>(t2565: F, t3783: F, t519: F, t2539: F, t3762: F, t571: F, t10313: F, t2553: F, t2554: F, t3745: F, t18557: F, t18561: F, t18563: F, t18565: F, t18567: F, t18569: F, t18573: F, t18576: F, t18578: F, t18583: F, t18585: F, t18589: F, t18591: F) -> (F, F, F, F, F) {
    let t18593 = t519 * t3783 * t2565;
    let t18594 = 16.0 / 405.0 * t18593;
    let t18596 = t571 * t3762 * t2539;
    let t18597 = 8.0 / 405.0 * t18596;
    let t18599 = t519 * t10313 * t2553;
    let t18600 = 8.0 / 243.0 * t18599;
    let t18602 = 8.0 / 27.0 * t3745 * t2554;
    let t18603 = -t18557 + t18561 + t18563 + t18565 + t18567 + t18569 - t18573 + t18576 - t18578 + t18583 - t18585 + t18589 + t18591 + t18594 - t18597 - t18600 + t18602;
    (t18594, t18597, t18600, t18602, t18603)
}
