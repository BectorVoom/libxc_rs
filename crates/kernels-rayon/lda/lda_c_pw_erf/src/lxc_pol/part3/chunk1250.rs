//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1250/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1250(t127: f64, t138: f64, t14563: f64, t14564: f64, t14565: f64, t14571: f64, t14572: f64, t14579: f64, t14641: f64, t14644: f64, t14648: f64, t14652: f64, t14661: f64, t14673: f64, t14720: f64, t14724: f64, t14729: f64, t14732: f64, t14734: f64, t14756: f64, t14771: f64, t14777: f64, t14804: f64, t14839: f64, t14853: f64, t1706: f64, t1712: f64, t1864: f64, t1878: f64, t3222: f64, t3332: f64, t3339: f64, t3340: f64, t3343: f64, t444: f64, t5621: f64, t5630: f64, t5633: f64, t5636: f64, t5667: f64, t756: f64, t8862: f64, t8865: f64, t8867: f64, t8869: f64, t8871: f64, t8873: f64, t8877: f64, t8894: f64, t8899: f64, t8917: f64, t8921: f64, t8932: f64, t8936: f64, t8940: f64, t8943: f64, t9019: f64, t9022: f64, t9025: f64, t9037: f64, t9054: f64, t9059: f64) -> f64 {
    let t14871 = -t444 * (t14565 + t14579 + t14673 + t14720) - 6.0_f64 * t14724 * t3340 + 6.0_f64 * t5621 * t3343 + 6.0_f64 * t14729 * t1712 + (t14564 + t14853 - t14572 + t8873 + t8877 - t14644 - t8936 - t14641 + 11.75232_f64 * t8921 + t14771 + t14756 - t8869 + t8871 + t14839 + t14804 - 2.93808_f64 * t8899 + 14.0_f64 / 9.0_f64 * t8940 + t14563 + 14.0_f64 / 27.0_f64 * t14732 + 2.2851733333333333_f64 * t14734 + t8865 + t8867 - 18.0_f64 * t14777 * t14661 - t14648 - 8.81424_f64 * t8894 + 2.0_f64 * t9025 + 14.6904_f64 * t8917 + t14652 + 6.85552_f64 * t9019 + 2.0_f64 * t9022 - t8862 + 176.2848_f64 * t127 * t9037 * t756 * t3222 + t8932 - 2.0_f64 / 3.0_f64 * t8943 - t14571) * t138 - 18.0_f64 * t3339 * t1878 * t1712 - 3.0_f64 * t1706 * t5667 + 12.0_f64 * t3332 * t5633 + 6.0_f64 * t3332 * t5636 + 6.0_f64 * t9054 * t1864 - 18.0_f64 * t9059 * t5630;
    t14871
}
