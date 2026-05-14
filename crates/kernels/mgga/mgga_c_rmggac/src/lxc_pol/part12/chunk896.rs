//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 896/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk896<F: Float>(t41130: F, t41316: F, t41301: F, t8750: F, t36110: F, t41304: F, t41307: F, t7603: F, t36103: F, t41310: F, t41313: F, t25607: F, t27: F, t3851: F, t39688: F, t41294: F, t41298: F, t41300: F, t41303: F, t41305: F, t41308: F, t41311: F, t41315: F) -> (F,) {
    let t41317 = t41130 * t41316;
    let t41319 = t8750 * t41301;
    let t41320 = 0.2419210303588817044e-2 * t41319;
    let t41321 = t36110 * t41304;
    let t41323 = t7603 * t41307;
    let t41324 = 0.33868944250243438616e-2 * t41323;
    let t41325 = t36103 * t41310;
    let t41327 = t7603 * t41313;
    let t41329 = t25607 * t27;
    let t41330 = t41329 * t41316;
    let t41332 = t3851 * t39688;
    let t41334 = 0.84672360625608596544e-3 * t41294 - t41298 - t41300 - t41303 + 0.68186654135613354325e-1 * t41305 + 0.72732431077987577946e-1 * t41308 - 0.2727466165424534173e-1 * t41311 + t41315 - 0.13637330827122670865e0 * t41317 - t41320 + 0.50803416375365157924e-2 * t41321 + t41324 - 0.31752135234603223704e-2 * t41325 + 0.33868944250243438618e-2 * t41327 - 0.7620512456304773689e-2 * t41330 + 0.2993560425465952141e-1 * t41332;
    (t41334,)
}
