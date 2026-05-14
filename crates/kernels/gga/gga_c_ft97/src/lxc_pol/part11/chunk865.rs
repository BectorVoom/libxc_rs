//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 865/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk865<F: Float>(t37723: F, t37725: F, t37728: F, t37733: F, t37736: F, t37739: F, t37742: F, t37745: F, t37752: F, t37756: F, t37758: F, t37820: F, t37795: F, t37798: F, t37800: F, t37802: F, t37806: F, t37808: F, t37812: F, t37816: F, t37824: F, t37828: F) -> (F, F) {
    let t40012 = 0.13039253546995884774e1 * t37723 + 0.14224640233086419754e1 * t37725 - 0.17780800291358024693e0 * t37728 - 0.62232801019753086422e0 * t37733 - 0.22226000364197530866e-1 * t37736 - 0.29634667152263374488e-1 * t37739 + 0.22226000364197530865e-1 * t37742 + 0.69147556688614540471e-1 * t37745 + 0.17286889172153635117e0 * t37752 + 0.16669500273148148149e-1 * t37756 - 0.10668480174814814815e1 * t37758;
    let t40033 = 0.4939111192043895748e-1 * t37820;
    let t40036 = 0.35561600582716049384e0 * t37795 + 0.44452000728395061732e-1 * t37798 - 0.2370773372181069959e0 * t37800 - 0.55318045350891632375e0 * t37802 + 0.66678001092592592595e-1 * t37806 - 0.17780800291358024692e0 * t37808 - 0.65196267734979423872e0 * t37812 - 0.19756444768175582992e0 * t37816 - t40033 + 0.12930593100770919068e2 * t37824 - 0.30424924942990397807e1 * t37828;
    (t40012, t40036)
}
