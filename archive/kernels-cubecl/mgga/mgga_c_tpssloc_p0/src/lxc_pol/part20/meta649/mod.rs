//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2388;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta649<F: Float>(t10599: F, t2799: F, t4370: F, t10595: F, t10596: F, t1547: F, t41935: F, t41942: F, t41887: F, t41889: F, t48134: F, t48137: F, t48142: F, t48145: F, t48148: F, t49009: F, t2807: F, t896: F, t13637: F, t41680: F, t41713: F, t47777: F, t48153: F, t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F) -> (F, F, F, F, F, F, F, F) {
        let (t49012, t49015, t49018, t49021, t49026) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2388::<F>(t10599, t2799, t4370, t10595, t10596, t1547, t41935, t41942, t41887, t41889, t48134, t48137, t48142, t48145, t48148, t49009);
        let (t49039, t49040, t49042) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2389::<F>(t2807, t896, t13637, t41680, t41713, t47777, t48153, t48155, t48157, t48159, t48161, t48163, t48165, t48167);
    (t49012, t49015, t49018, t49021, t49026, t49039, t49040, t49042)
}
