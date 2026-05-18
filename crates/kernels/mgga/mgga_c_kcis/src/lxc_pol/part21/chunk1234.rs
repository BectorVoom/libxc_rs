//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1234/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1234<F: Float>(t26711: F, t9386: F, t26966: F, t27023: F, t27069: F, t3489: F, t15216: F, t26956: F, t26955: F, t15573: F, t26998: F, t7788: F) -> (F, F, F, F, F, F, F) {
    let t92820 = t9386 * t26711;
    let t92822 = t26966 * t27023;
    let t92830 = t27069 * t3489;
    let t92850 = t15216 * t26956;
    let t92851 = t26955 * t92850;
    let t92860 = t15573 * t26998;
    let t92861 = t7788 * t92860;
    (t92820, t92822, t92830, t92850, t92851, t92860, t92861)
}
