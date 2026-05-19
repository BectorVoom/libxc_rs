//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1007/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1007<F: Float>(t25636: F, t40901: F, t2347: F, t25525: F, t794: F, t3839: F, t40905: F, t25518: F, t38564: F, t41132: F, t41134: F, t41136: F, t41138: F, t41140: F, t41142: F, t41144: F, t41146: F, t41148: F, t41151: F, t41153: F) -> F {
    let t41155 = t25636 * t40901;
    let t41158 = t25525 * t2347 * t794;
    let t41160 = t3839 * t40905;
    let t41162 = t25518 * t38564;
    let t41164 = F::cast_from(0.1814407727691612783e-2_f64) * t41132 + F::cast_from(0.5987120850931904282e-1_f64) * t41134 + F::cast_from(0.5987120850931904282e-1_f64) * t41136 + F::cast_from(0.5987120850931904282e-1_f64) * t41138 + F::cast_from(0.2993560425465952141e-1_f64) * t41140 - F::cast_from(0.13276154105060581339e-2_f64) * t41142 - F::cast_from(0.5987120850931904282e-1_f64) * t41144 - F::cast_from(0.15965655602485078085e0_f64) * t41146 + F::cast_from(0.2993560425465952141e0_f64) * t41148 - F::cast_from(0.5454932330849068346e-1_f64) * t41151 + F::cast_from(0.13637330827122670865e0_f64) * t41153 + F::cast_from(0.22303938896501776649e-1_f64) * t41155 - F::cast_from(0.39828462315181744017e-2_f64) * t41158 + F::cast_from(0.70806155226989767141e-2_f64) * t41160 - F::cast_from(0.13939961810313610406e-1_f64) * t41162;
    t41164
}
