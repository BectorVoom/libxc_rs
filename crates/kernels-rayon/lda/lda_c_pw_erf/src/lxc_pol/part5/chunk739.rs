//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 739/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk739(t4730: f64, t4902: f64, t4908: f64, t1449: f64, t2565: f64, t519: f64, t1475: f64, t2561: f64, t571: f64, t3959: f64, t3960: f64, t3963: f64, t4790: f64, t4793: f64, t4797: f64, t4836: f64, t4879: f64, t4891: f64, t4905: f64, t4917: f64, t4935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6678 = 8.0_f64 / 135.0_f64 * t4730;
    let t6680 = 32.0_f64 / 135.0_f64 * t4902;
    let t6681 = 32.0_f64 / 135.0_f64 * t4908;
    let t6682 = t1449 * t2565;
    let t6683 = t519 * t6682;
    let t6684 = 16.0_f64 / 135.0_f64 * t6683;
    let t6685 = t1475 * t2561;
    let t6686 = t571 * t6685;
    let t6687 = 16.0_f64 / 135.0_f64 * t6686;
    let t6688 = t6678 - t4790 - t4793 + t4797 - t4836 - t3959 + 0.033245444444444446_f64 * t3960 + t3963 + t4879 + t4891 - t6680 + t4905 + t6681 + t4917 - t4935 - t6684 - t6687;
    (t6678, t6680, t6681, t6682, t6683, t6684, t6685, t6686, t6687, t6688)
}
