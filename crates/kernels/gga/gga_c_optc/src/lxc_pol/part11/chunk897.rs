//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 897/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk897<F: Float>(t1583: F, t17648: F, t1582: F, t12168: F, t5171: F, t1056: F, t17426: F, t3018: F, t1220: F, t15008: F, t15016: F, t15064: F, t15083: F, t15179: F, t15181: F, t15200: F, t15205: F, t1575: F, t1579: F, t1588: F, t17615: F, t17619: F, t17623: F, t17627: F, t17635: F, t17645: F, t4536: F, t498: F, t5229: F, t5233: F, t5246: F, t5474: F) -> (F, F, F, F, F, F) {
    let t17649 = t1583 * t17648;
    let t17650 = t1582 * t17649;
    let t17655 = 6.0 * t12168 * t5171;
    let t17656 = t17426 * t1056;
    let t17658 = 6.0 * t3018 * t17656;
    let t17659 = -4.0 * t5474 * t1575 + t17615 * t498 / 2.0 + 50.0 / 9.0 * t17619 * t1588 + 20000.0 / 27.0 * t17623 * t5246 + 20000.0 / 81.0 * t15064 * t17627 + t15179 / 6.0 + t15181 / 3.0 - 50.0 / 3.0 * t15083 * t5229 + t1220 * t17635 / 6.0 - 8.0 / 3.0 * t15008 * t1579 - t4536 * t5233 + 44.0 / 9.0 * t15016 * t1579 - t17645 + t15200 / 2.0 + 34100.0 / 243.0 * t17650 * t1588 + 44.0 / 9.0 * t15205 - t17655 + t17658;
    (t17649, t17650, t17655, t17656, t17658, t17659)
}
