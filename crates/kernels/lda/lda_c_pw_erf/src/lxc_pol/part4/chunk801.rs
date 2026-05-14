//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 801/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk801<F: Float>(t3908: F, t3910: F, t3917: F, t3919: F, t3923: F, t3929: F, t3935: F, t3938: F, t3944: F, t3947: F, t3950: F, t3951: F, t4710: F, t4721: F, t4726: F, t4728: F, t4731: F) -> (F,) {
    let t5848 = -t4710 + 2.0 / 9.0 * t3908 + 8.0 / 9.0 * t3910 + t3917 + t3919 + t3923 + t3929 + t3935 - t3938 + t3944 / 3.0 + 0.06077777777777778 * t3947 + 2.0 / 3.0 * t3950 + 0.2431111111111111 * t3951 + t4721 + t4726 - t4728 + t4731;
    (t5848,)
}
