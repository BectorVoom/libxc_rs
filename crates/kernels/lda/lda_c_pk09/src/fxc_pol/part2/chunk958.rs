//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 958/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk958<F: Float>(t11076: F, t11102: F, t11208: F, t11211: F, t11216: F, t11220: F, t11223: F, t11227: F, t11230: F, t11594: F, t11600: F, t11610: F, t11632: F, t11634: F, t11657: F, t11688: F, t11691: F, t11700: F, t11702: F, t11704: F, t11706: F, t11708: F, t11759: F, t11805: F, t11837: F, t11840: F, t11843: F, t11846: F, t11848: F, t11850: F, t11852: F, t11854: F, t11857: F, t11956: F, t2032: F, t2037: F, t2832: F, t2835: F, t455: F, t6467: F, t6816: F, t6823: F, t6827: F, t6829: F, t6989: F, t7091: F, t7098: F, t7103: F, t7137: F, t7221: F, t7267: F, t7313: F, t7321: F) -> (F,) {
    let t11960 = -t7221 / 18.0 - t7103 / 6.0 + t11691 / 6.0 + t11956 + 0.29951248675449116 * t11208 - 0.07400578449205193 * t11211 + t11805 - t11700 / 18.0 - 0.07400578449205193 * t11227 - 0.07400578449205193 * t11230 + t11594 + t11688 - 0.03412591035063918 * t6467 + t7313 / 6.0 - t11852 / 18.0 + t11708 / 6.0 + 0.07400578449205193 * t6816 - t7321 / 18.0 - t11702 / 18.0 + t11704 / 18.0 + t11706 / 18.0 - 0.10237773105191754 * t11076 + t11657 + t7267 * t11634 / 12.0 - t11600 * t455 / 6.0 - t11854 * t455 / 6.0 - t11857 * t455 / 6.0 + t2832 * t7137 / 12.0 - t11837 * t455 / 6.0 - t11840 * t455 / 6.0 - t11843 * t455 / 6.0 + 0.1110086767380779 * t11216 + 0.07400578449205193 * t11223 - t6989 + t7098 + t7091 / 18.0 + t11632 + 0.14975624337724558 * t11220 - 0.04991874779241519 * t6829 + t11610 + t11759 - 0.07400578449205193 * t6827 + t2835 * t2032 / 6.0 - t2037 * t11102 / 6.0 + 0.14975624337724558 * t6823 + t11846 / 18.0 - t11848 / 18.0 - t11850 / 18.0;
    (t11960,)
}
