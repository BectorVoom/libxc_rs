//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta946 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3183;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta946<F: Float>(t43766: F, t44361: F, t12916: F, t17419: F, t5340: F, t45608: F, t58919: F, t45786: F, t17708: F, t45846: F, t12975: F, t1803: F, t225: F, t56412: F, t480: F, t12984: F, t5323: F, t17390: F, t3718: F, t17500: F, t372: F, t13142: F, t56878: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t58983, t58997, t59001, t59011, t59017, t59025) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3183::<F>(t43766, t44361, t12916, t17419, t5340, t45608, t58919, t45786, t17708, t45846, t12975, t1803);
        let (t59032, t59033, t59040, t59043, t59062, t59066) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3184::<F>(t225, t56412, t480, t12984, t5323, t12916, t17390, t3718, t17500, t372, t13142, t56878);
    (t58983, t58997, t59001, t59011, t59017, t59025, t59032, t59033, t59040, t59043, t59062, t59066)
}
