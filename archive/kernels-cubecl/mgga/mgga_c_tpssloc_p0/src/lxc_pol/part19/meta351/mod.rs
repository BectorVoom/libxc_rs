//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta351<F: Float>(t2403: F, t2830: F, t10317: F, t699: F, t136: F, t2826: F, t41697: F, t41701: F, t41709: F, t908: F, t41640: F, t41642: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F) -> (F, F, F, F, F, F, F) {
        let (t41831, t41833, t41836, t41839, t41842, t41845, t41855) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1277::<F>(t2403, t2830, t10317, t699, t136, t2826, t41697, t41701, t41709, t908, t41640, t41642, t41656, t41658, t41660, t41662, t41669, t41673, t41675);
    (t41831, t41833, t41836, t41839, t41842, t41845, t41855)
}
