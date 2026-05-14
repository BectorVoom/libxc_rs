//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1128/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1128<F: Float>(t27348: F, t8144: F, t28397: F, t28339: F, t3728: F, t7898: F, t98524: F, t27410: F, t28426: F, t3245: F, t8176: F, t27345: F, t1014: F, t28409: F, t97997: F, t27563: F, t28727: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t98909 = 0.46336805555555555556e-3 * t8144 * t27348;
    let t98911 = 0.61836467013888888889e-4 * t28397 * t27348;
    let t98918 = t3728 * t28339;
    let t98934 = t7898 * t98524;
    let t98938 = t27410 * t28426;
    let t98942 = t3245 * t8176;
    let t98945 = 0.46336805555555555556e-3 * t8144 * t27345;
    let t98946 = t1014 * t28409;
    let t98978 = 0.15476481481481481481e-2 * t97997;
    let t98986 = 0.61782407407407407408e-3 * t28727 * t27563;
    (t98909, t98911, t98918, t98934, t98938, t98942, t98945, t98946, t98978, t98986)
}
