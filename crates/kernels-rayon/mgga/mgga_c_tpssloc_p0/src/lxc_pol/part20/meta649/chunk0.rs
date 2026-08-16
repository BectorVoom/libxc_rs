//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2388/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2388(t10599: f64, t2799: f64, t4370: f64, t10595: f64, t10596: f64, t1547: f64, t41935: f64, t41942: f64, t41887: f64, t41889: f64, t48134: f64, t48137: f64, t48142: f64, t48145: f64, t48148: f64, t49009: f64) -> (f64, f64, f64, f64, f64) {
    let t49012 = t10599 * t4370 * t2799;
    let t49015 = t10595 * t4370 * t2799;
    let t49018 = t41935 * t1547 * t10596;
    let t49021 = t41942 * t1547 * t10596;
    let t49026 = -0.27385555555555555556e-1_f64 * t48134 - 0.85199506172839506175e-1_f64 * t48137 + 0.49293999999999999999e0_f64 * t48142 - 0.147882e1_f64 * t48145 - 0.9494625e0_f64 * t49009 - 0.230371875e0_f64 * t49012 + 0.427258125e1_f64 * t49015 - 0.3560484375e1_f64 * t49018 + 0.1151859375e0_f64 * t49021 - 0.10954222222222222222e0_f64 * t48148 - 0.32862666666666666666e0_f64 * t41887 + 0.54771111111111111111e-1_f64 * t41889;
    (t49012, t49015, t49018, t49021, t49026)
}
