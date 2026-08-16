//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3110/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3110<F: Float>(t12784: F, t17384: F, t12772: F, t17668: F, t3625: F, t17673: F, t12910: F, t12916: F, t17460: F, t17213: F, t3172: F, t5384: F) -> (F, F, F, F, F) {
    let t57164 = t12784 * t17384;
    let t57167 = t3625 * t12772 * t17668;
    let t57170 = t3625 * t12772 * t17673;
    let t57173 = t12910 * t12916 * t17460;
    let t57176 = t5384 * t3172 * t17213;
    (t57164, t57167, t57170, t57173, t57176)
}
