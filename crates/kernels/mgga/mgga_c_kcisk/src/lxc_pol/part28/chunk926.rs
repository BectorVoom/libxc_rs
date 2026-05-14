//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 926/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk926<F: Float>(t2558: F, t5060: F, t140: F, t2554: F, t430: F, t299: F, t7394: F, t11: F, t139: F, t41: F, t7379: F, t7371: F, t4663: F, t725: F, t10459: F, t702: F) -> (F, F, F, F, F, F, F) {
    let t17982 = t2558 * t5060;
    let t18005 = t140 * t430 * t2554;
    let t18031 = 0.53062222222222222222e-1 * t140 * t299 * t7394;
    let t18053 = t139 * t11 * t41;
    let t18054 = t18053 * t7379;
    let t18057 = 0.5895802469135802469e-1 * t18053 * t7371;
    let t18063 = t725 * t4663;
    let t18076 = t10459 * t702;
    (t17982, t18005, t18031, t18054, t18057, t18063, t18076)
}
