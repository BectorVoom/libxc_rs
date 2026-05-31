//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 230/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk230<F: Float>(t781: F, t142: F, t143: F, t684: F, t126: F, t60: F, t15: F, t130: F, t2: F, t4: F, t88: F, t128: F, t97: F) -> (F, F, F, F, F, F, F, F) {
    let t782 = F::cast_from(1.0_f64) / t781;
    let t783 = t142 * t782;
    let t784 = t684 * t143;
    let t787 = t60 * t126;
    let t788 = t787 * t15;
    let t789 = t130 * t2;
    let t790 = t4 * t88;
    let t791 = t789 * t790;
    let t794 = t128 * t97;
    (t782, t783, t784, t787, t788, t789, t791, t794)
}
