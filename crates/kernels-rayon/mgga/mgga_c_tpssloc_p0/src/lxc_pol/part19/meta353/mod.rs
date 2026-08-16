//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1280;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1281;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta353(t10213: f64, t241: f64, t136: f64, t41667: f64, t41671: f64, t908: f64, t10319: f64, t699: f64, t10313: f64, t2826: f64, t41649: f64, t41654: f64, t41642: f64, t41646: f64, t41651: f64, t41656: f64, t41658: f64, t41660: f64, t41662: f64, t41669: f64, t41673: f64, t41675: f64, t41678: f64, t41680: f64, t41682: f64, t41684: f64, t41690: f64, t41695: f64, t41699: f64, t41703: f64, t41707: f64, t41711: f64, t41713: f64, t41717: f64, t894: f64, t901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41882, t41885, t41887, t41889, t41892, t41904) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1280(t10213, t241, t136, t41667, t41671, t908, t10319, t699, t10313, t2826, t41649, t41654);
        let (t41912, t41925) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1281(t41642, t41646, t41651, t41656, t41658, t41660, t41662, t41669, t41673, t41675, t41904, t41678, t41680, t41682, t41684, t41690, t41695, t41699, t41703, t41707, t41711, t41713, t41717);
        let (t41927, t41929, t41931) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1282(t41912, t41925, t894, t901, t41646, t41651, t41680, t41695, t41707, t41713, t41717, t41882, t41885, t41887, t41889, t41892);
    (t41882, t41885, t41887, t41889, t41892, t41927, t41929, t41931)
}
