//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3184/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3184<F: Float>(t225: F, t56412: F, t480: F, t12984: F, t5323: F, t12916: F, t17390: F, t3718: F, t17500: F, t372: F, t13142: F, t56878: F) -> (F, F, F, F, F, F) {
    let t59032 = t56412 * t225;
    let t59033 = t59032 * t480;
    let t59040 = t5323 * t12984;
    let t59043 = t3718 * t12916 * t17390;
    let t59062 = t372 * t17500;
    let t59066 = t13142 * t56878;
    (t59032, t59033, t59040, t59043, t59062, t59066)
}
