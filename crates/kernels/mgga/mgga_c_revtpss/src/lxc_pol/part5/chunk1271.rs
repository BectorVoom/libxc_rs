//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1271/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1271<F: Float>(t14239: F, t5741: F, t6844: F, t72: F, t686: F, t4101: F, t6874: F, t10098: F, t10102: F, t10109: F, t10114: F, t14218: F, t14221: F, t14227: F, t14229: F, t14233: F, t14241: F, t14243: F, t22005: F, t5675: F, t5745: F) -> (F,) {
    let t22329 = t14239 * t5741;
    let t22331 = t6844 * t72;
    let t22332 = t22331 * t686;
    let t22333 = t4101 * t22332;
    let t22335 = t6874 * t72;
    let t22336 = t22335 * t686;
    let t22337 = t4101 * t22336;
    let t22344 = -t14218 - 0.23131639038696784278e-2 * t14221 - 0.13009920719177044025e-1 * t10098 + t10102 + t14227 - t14229 - t14233 - 0.19514881078765566037e-1 * t22329 - 0.9757440539382783019e-2 * t22333 - 0.9757440539382783019e-2 * t22337 - t14241 + 0.26019841438354088051e-1 * t14243 + 0.11565819519348392139e-2 * t10109 + t10114 + 0.39512695097613069591e1 * t5745 * t22005 * t5675;
    (t22344,)
}
