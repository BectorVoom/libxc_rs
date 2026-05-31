//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 838/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk838<F: Float>(t1325: F, t7702: F, t2429: F, t806: F, t3402: F, t519: F, t2419: F, t833: F, t1308: F, t571: F, t1319: F, t7426: F) -> (F, F, F, F, F, F, F, F) {
    let t7704 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1325 * t7702;
    let t7705 = t2429 * t806;
    let t7706 = t3402 * t7705;
    let t7708 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t519 * t7706;
    let t7709 = t2419 * t833;
    let t7710 = t1308 * t7709;
    let t7712 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t571 * t7710;
    let t7713 = t1319 * t7426;
    (t7704, t7705, t7706, t7708, t7709, t7710, t7712, t7713)
}
