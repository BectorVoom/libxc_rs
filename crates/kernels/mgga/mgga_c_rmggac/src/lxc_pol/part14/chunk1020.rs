//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1020/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1020<F: Float>(t41307: F, t7603: F, t36103: F, t41310: F, t41313: F, t25607: F, t27: F, t41316: F, t3851: F, t39688: F, t41294: F, t41298: F, t41300: F, t41303: F, t41305: F, t41308: F, t41311: F, t41315: F, t41317: F, t41320: F, t41321: F) -> F {
    let t41323 = t7603 * t41307;
    let t41324 = F::cast_from(0.33868944250243438616e-2_f64) * t41323;
    let t41325 = t36103 * t41310;
    let t41327 = t7603 * t41313;
    let t41329 = t25607 * t27;
    let t41330 = t41329 * t41316;
    let t41332 = t3851 * t39688;
    let t41334 = F::cast_from(0.84672360625608596544e-3_f64) * t41294 - t41298 - t41300 - t41303 + F::cast_from(0.68186654135613354325e-1_f64) * t41305 + F::cast_from(0.72732431077987577946e-1_f64) * t41308 - F::cast_from(0.2727466165424534173e-1_f64) * t41311 + t41315 - F::cast_from(0.13637330827122670865e0_f64) * t41317 - t41320 + F::cast_from(0.50803416375365157924e-2_f64) * t41321 + t41324 - F::cast_from(0.31752135234603223704e-2_f64) * t41325 + F::cast_from(0.33868944250243438618e-2_f64) * t41327 - F::cast_from(0.7620512456304773689e-2_f64) * t41330 + F::cast_from(0.2993560425465952141e-1_f64) * t41332;
    t41334
}
