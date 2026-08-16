//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1280;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1281;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta353<F: Float>(t10213: F, t241: F, t136: F, t41667: F, t41671: F, t908: F, t10319: F, t699: F, t10313: F, t2826: F, t41649: F, t41654: F, t41642: F, t41646: F, t41651: F, t41656: F, t41658: F, t41660: F, t41662: F, t41669: F, t41673: F, t41675: F, t41678: F, t41680: F, t41682: F, t41684: F, t41690: F, t41695: F, t41699: F, t41703: F, t41707: F, t41711: F, t41713: F, t41717: F, t894: F, t901: F) -> (F, F, F, F, F, F, F, F) {
        let (t41882, t41885, t41887, t41889, t41892, t41904) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1280::<F>(t10213, t241, t136, t41667, t41671, t908, t10319, t699, t10313, t2826, t41649, t41654);
        let (t41912, t41925) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1281::<F>(t41642, t41646, t41651, t41656, t41658, t41660, t41662, t41669, t41673, t41675, t41904, t41678, t41680, t41682, t41684, t41690, t41695, t41699, t41703, t41707, t41711, t41713, t41717);
        let (t41927, t41929, t41931) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1282::<F>(t41912, t41925, t894, t901, t41646, t41651, t41680, t41695, t41707, t41713, t41717, t41882, t41885, t41887, t41889, t41892);
    (t41882, t41885, t41887, t41889, t41892, t41927, t41929, t41931)
}
