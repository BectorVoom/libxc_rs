//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1076/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1076<F: Float>(t11267: F, t11282: F, t7192: F, t7273: F, t9271: F, t9366: F, t939: F, t4296: F, t7282: F, t941: F, t3507: F, t3513: F, t11257: F, t11259: F, t11262: F, t7384: F, t9433: F) -> (F, F, F, F, F, F) {
    let t11283 = -t7273 + 4.0 / 9.0 * t7192 + 8.0 / 9.0 * t9271 - t9366 - t11267 / 3.0 + t11282;
    let t11284 = t939 * t11283;
    let t11290 = t7282 * t4296;
    let t11291 = t11290 * t941;
    let t11293 = t3513 * t3507;
    let t11295 = 0.142419375e1 * t11257 - 0.1898925e1 * t11259 - 0.9494625e0 * t11262 + 0.1898925e1 * t11284 - t7384 + 0.39862222222222222223e0 * t7192 + 0.79724444444444444445e0 * t9271 - t9433 - 0.29896666666666666667e0 * t11267 + 0.8969e0 * t11282 - 0.76790625e-1 * t11291 + 0.3071625e0 * t11293;
    (t11283, t11284, t11290, t11291, t11293, t11295)
}
