//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 186/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk186<F: Float>(t142: F, t782: F, t143: F, t684: F, t126: F, t60: F, t15: F, t130: F, t2: F, t4: F, t88: F, t128: F, t97: F, t728: F, t11: F, t122: F, t144: F, t145: F, t148: F, t745: F, t85: F) -> (F, F, F, F, F, F, F, F, F) {
    let t783 = t142 * t782;
    let t784 = t684 * t143;
    let t787 = t60 * t126;
    let t788 = t787 * t15;
    let t789 = t130 * t2;
    let t790 = t4 * t88;
    let t791 = t789 * t790;
    let t794 = t128 * t97;
    let t795 = t794 * t728;
    let t804 = 0.619125e-2 * t784 * t145 - 0.123825e-1 * t788 * t791 - 0.619125e-2 * t144 * t795 - 0.53062222222222222221e-1 * t85 * t11 * t122 - 0.79593333333333333331e-1 * t85 * t148 * t745;
    (t783, t784, t787, t788, t789, t791, t794, t795, t804)
}
