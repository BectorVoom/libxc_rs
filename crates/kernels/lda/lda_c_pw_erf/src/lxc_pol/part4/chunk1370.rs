//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1370/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1370<F: Float>(t15407: F, t15431: F, t15442: F, t15463: F, t15477: F, t19885: F, t19890: F, t19892: F, t2705: F, t2740: F, t2743: F, t4385: F, t4387: F, t4389: F, t4391: F, t4394: F, t4398: F, t4403: F, t4406: F, t5968: F, t5976: F, t7: F, t8097: F, t8098: F, t8099: F, t8101: F, t8102: F, t8103: F) -> (F,) {
    let t19908 = 2.0 * t4385 + t7 * (t15407 + t15431 + t15442 + t15463 + t15477 + t19885 + t19890 + t19892) - t8097 - 1.169644679491041 * t5968 + t8098 + t8099 + 0.06506148529668915 * t2705 - t8101 - t8102 + 0.0014649244029402528 * t4387 - 3.5089340384731225 * t4389 - 103.89453539625518 * t4391 - 4.678578717964164 * t4394 - t8103 - 3.5089340384731225 * t2740 + 0.06506148529668915 * t4398 - 1.169644679491041 * t2743 - t5976 - 64.0 * t4403 + 6.0 * t4406;
    (t19908,)
}
