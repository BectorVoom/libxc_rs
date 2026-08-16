//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 665/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk665<F: Float>(t397: F, t5701: F, t3010: F, t3158: F, t3161: F, t3169: F, t3173: F, t1880: F, t405: F, t455: F, t5495: F, t39: F, t865: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5702 = t5701 * t397;
    let t5703 = F::cast_from(0.0003662311007350632_f64) * t5702;
    let t5704 = F::cast_from(4.0_f64) * t3010;
    let t5707 = F::cast_from(48.0_f64) * t3158;
    let t5708 = F::cast_from(80.0_f64) * t3161;
    let t5709 = F::cast_from(12.0_f64) * t3169;
    let t5711 = F::cast_from(32.0_f64) * t3173;
    let t5735 = t405 * t1880;
    let t5740 = t455 * t5495;
    let t5745 = t39 * t865;
    (t5702, t5703, t5704, t5707, t5708, t5709, t5711, t5735, t5740, t5745)
}
