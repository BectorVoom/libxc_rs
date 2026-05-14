//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1076/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1076<F: Float>(t26955: F, t96975: F, t15220: F, t26960: F, t28124: F, t15216: F, t28117: F, t28189: F, t3489: F, t28203: F, t15573: F, t28131: F, t7788: F, t96727: F, t27014: F, t28214: F) -> (F, F, F, F, F, F, F, F, F) {
    let t96977 = 0.10306077835648148148e-4 * t26955 * t96975;
    let t96980 = 0.10297067901234567901e-3 * t26960 * t15220 * t28124;
    let t96993 = 0.15445601851851851852e-3 * t26960 * t15216 * t28117;
    let t97010 = t28189 * t3489;
    let t97015 = t28203 * t3489;
    let t97024 = t15573 * t28131;
    let t97026 = 0.23168402777777777778e-3 * t7788 * t97024;
    let t97028 = 0.46336805555555555556e-3 * t7788 * t96727;
    let t97030 = 0.7722800925925925926e-4 * t27014 * t28214;
    (t96977, t96980, t96993, t97010, t97015, t97024, t97026, t97028, t97030)
}
