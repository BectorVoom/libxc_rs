//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 854/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk854<F: Float>(t41313: F, t7599: F, t5223: F, t649: F, t41130: F, t41301: F, t8750: F, t36110: F, t41304: F, t41307: F, t7603: F, t36103: F, t41310: F, t25607: F, t27: F, t3851: F, t39688: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41314 = t7599 * t41313;
    let t41316 = t649 * t5223;
    let t41317 = t41130 * t41316;
    let t41319 = t8750 * t41301;
    let t41321 = t36110 * t41304;
    let t41323 = t7603 * t41307;
    let t41325 = t36103 * t41310;
    let t41327 = t7603 * t41313;
    let t41329 = t25607 * t27;
    let t41330 = t41329 * t41316;
    let t41332 = t3851 * t39688;
    (t41314, t41317, t41319, t41321, t41323, t41325, t41327, t41330, t41332)
}
