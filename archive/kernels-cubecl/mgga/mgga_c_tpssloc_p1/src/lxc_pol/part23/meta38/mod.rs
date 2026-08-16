//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta38 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk270;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk271;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk272;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk273;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta38<F: Float>(t244: F, t68: F, t590: F, t61: F, t241: F, t248: F, t238: F, t234: F, t236: F, t240: F, t812: F, t200: F, t243: F, t67: F, t225: F, t253: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t824, t835) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk270::<F>(t244, t68, t590, t61);
        let t836 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk271::<F>(t241, t835);
        let (t838, t840, t841, t842, t843) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk272::<F>(t244, t248, t836, t238, t234, t236, t240, t812);
        let t845 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk273::<F>(t200, t243);
        let (t847, t855) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk274::<F>(t241, t67, t845, t225, t253);
    (t824, t835, t836, t838, t840, t841, t842, t843, t845, t847, t855)
}
