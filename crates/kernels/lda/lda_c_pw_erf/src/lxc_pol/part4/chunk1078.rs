//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1078/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1078<F: Float>(t13829: F, t15704: F, t4506: F, t34: F, t4516: F, t13771: F, t4522: F, t2334: F, t549: F, t593: F, t3974: F, t352: F, t11914: F, t5155: F, t13115: F, t5166: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15707 = 64.0 / 81.0 * t4506 * t13829 * t15704;
    let t15708 = t4516 * t34;
    let t15711 = 32.0 / 27.0 * t13771 * t4522 * t15708;
    let t15712 = t2334 * t549;
    let t15713 = t15712 * t593;
    let t15716 = 16.0 / 27.0 * t3974 * t4522 * t15713;
    let t15717 = t15712 * t352;
    let t15720 = 128.0 / 81.0 * t3974 * t11914 * t15717;
    let t15721 = t5155 * t34;
    let t15724 = 64.0 / 27.0 * t13115 * t5166 * t15721;
    (t15707, t15708, t15711, t15713, t15716, t15717, t15720, t15721, t15724)
}
