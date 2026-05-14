//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 717/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk717<F: Float>(t101: F, t7213: F, t2363: F, t473: F, t483: F, t485: F, t4153: F, t4156: F, t4160: F, t4163: F, t4165: F, t4168: F, t4172: F, t4250: F, t4258: F, t5440: F, t5442: F, t5444: F, t5448: F, t5449: F, t5455: F, t5459: F) -> (F, F, F, F) {
    let t7214 = t101 * t7213;
    let t7220 = t473 * t2363;
    let t7222 = t7220 * t483 * t485;
    let t7231 = -0.04789693604101844 * t5440 - 0.001975389032890948 * t7222 - 0.12602162889256446 * t5442 - 0.06301081444628223 * t5444 + t5448 + 0.12602162889256446 * t5449 - t5455 + t5459 + t4258 - 0.02394846802050922 * t4250 - 0.003950778065781896 * t4153 - 0.0004954275694490498 * t4156 - t4160 - t4163 - t4165 + 0.006584630109636494 * t4168 + t4172;
    (t7214, t7220, t7222, t7231)
}
