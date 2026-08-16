//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 739/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk739<F: Float>(t4730: F, t4902: F, t4908: F, t1449: F, t2565: F, t519: F, t1475: F, t2561: F, t571: F, t3959: F, t3960: F, t3963: F, t4790: F, t4793: F, t4797: F, t4836: F, t4879: F, t4891: F, t4905: F, t4917: F, t4935: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6678 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t4730;
    let t6680 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t4902;
    let t6681 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t4908;
    let t6682 = t1449 * t2565;
    let t6683 = t519 * t6682;
    let t6684 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t6683;
    let t6685 = t1475 * t2561;
    let t6686 = t571 * t6685;
    let t6687 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t6686;
    let t6688 = t6678 - t4790 - t4793 + t4797 - t4836 - t3959 + F::cast_from(0.033245444444444446_f64) * t3960 + t3963 + t4879 + t4891 - t6680 + t4905 + t6681 + t4917 - t4935 - t6684 - t6687;
    (t6678, t6680, t6681, t6682, t6683, t6684, t6685, t6686, t6687, t6688)
}
