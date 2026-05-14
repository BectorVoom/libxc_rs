//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1085/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1085<F: Float>(t127: F, t138: F, t14563: F, t14564: F, t14565: F, t14571: F, t14572: F, t14579: F, t14641: F, t14644: F, t14648: F, t14652: F, t14661: F, t14673: F, t14720: F, t14724: F, t14729: F, t14732: F, t14734: F, t14756: F, t14771: F, t14777: F, t14804: F, t14839: F, t14853: F, t1706: F, t1712: F, t1864: F, t1878: F, t3222: F, t3332: F, t3339: F, t3340: F, t3343: F, t444: F, t5621: F, t5630: F, t5633: F, t5636: F, t5667: F, t756: F, t8862: F, t8865: F, t8867: F, t8869: F, t8871: F, t8873: F, t8877: F, t8894: F, t8899: F, t8917: F, t8921: F, t8932: F, t8936: F, t8940: F, t8943: F, t9019: F, t9022: F, t9025: F, t9037: F, t9054: F, t9059: F) -> (F,) {
    let t14871 = -t444 * (t14565 + t14579 + t14673 + t14720) - 6.0 * t14724 * t3340 + 6.0 * t5621 * t3343 + 6.0 * t14729 * t1712 + (t14564 + t14853 - t14572 + t8873 + t8877 - t14644 - t8936 - t14641 + 11.75232 * t8921 + t14771 + t14756 - t8869 + t8871 + t14839 + t14804 - 2.93808 * t8899 + 14.0 / 9.0 * t8940 + t14563 + 14.0 / 27.0 * t14732 + 2.2851733333333333 * t14734 + t8865 + t8867 - 18.0 * t14777 * t14661 - t14648 - 8.81424 * t8894 + 2.0 * t9025 + 14.6904 * t8917 + t14652 + 6.85552 * t9019 + 2.0 * t9022 - t8862 + 176.2848 * t127 * t9037 * t756 * t3222 + t8932 - 2.0 / 3.0 * t8943 - t14571) * t138 - 18.0 * t3339 * t1878 * t1712 - 3.0 * t1706 * t5667 + 12.0 * t3332 * t5633 + 6.0 * t3332 * t5636 + 6.0 * t9054 * t1864 - 18.0 * t9059 * t5630;
    (t14871,)
}
