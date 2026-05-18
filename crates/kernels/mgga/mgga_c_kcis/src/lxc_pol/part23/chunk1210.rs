//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1210/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1210<F: Float>(t17454: F, t27544: F, t28594: F, t4262: F, t3734: F, t6034: F, t17409: F, t7948: F, t11783: F, t2055: F, t17471: F, t28629: F) -> (F, F, F, F, F, F) {
    let t97742 = t27544 * t17454;
    let t97744 = t28594 * t4262;
    let t97746 = t3734 * t6034;
    let t97748 = t7948 * t17409;
    let t97750 = t11783 * t2055;
    let t97752 = t28629 * t17471;
    (t97742, t97744, t97746, t97748, t97750, t97752)
}
