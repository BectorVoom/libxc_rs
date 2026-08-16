//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2388;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta649(t10599: f64, t2799: f64, t4370: f64, t10595: f64, t10596: f64, t1547: f64, t41935: f64, t41942: f64, t41887: f64, t41889: f64, t48134: f64, t48137: f64, t48142: f64, t48145: f64, t48148: f64, t49009: f64, t2807: f64, t896: f64, t13637: f64, t41680: f64, t41713: f64, t47777: f64, t48153: f64, t48155: f64, t48157: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49012, t49015, t49018, t49021, t49026) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2388(t10599, t2799, t4370, t10595, t10596, t1547, t41935, t41942, t41887, t41889, t48134, t48137, t48142, t48145, t48148, t49009);
        let (t49039, t49040, t49042) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2389(t2807, t896, t13637, t41680, t41713, t47777, t48153, t48155, t48157, t48159, t48161, t48163, t48165, t48167);
    (t49012, t49015, t49018, t49021, t49026, t49039, t49040, t49042)
}
