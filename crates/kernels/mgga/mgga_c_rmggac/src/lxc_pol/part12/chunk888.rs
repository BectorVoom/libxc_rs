//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 888/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk888<F: Float>(t41132: F, t41134: F, t41136: F, t41138: F, t41140: F, t41142: F, t41144: F, t41146: F, t41148: F, t41151: F, t41153: F, t41155: F, t41158: F, t41160: F, t41162: F, t25640: F, t36: F) -> (F, F) {
    let t41164 = 0.1814407727691612783e-2 * t41132 + 0.5987120850931904282e-1 * t41134 + 0.5987120850931904282e-1 * t41136 + 0.5987120850931904282e-1 * t41138 + 0.2993560425465952141e-1 * t41140 - 0.13276154105060581339e-2 * t41142 - 0.5987120850931904282e-1 * t41144 - 0.15965655602485078085e0 * t41146 + 0.2993560425465952141e0 * t41148 - 0.5454932330849068346e-1 * t41151 + 0.13637330827122670865e0 * t41153 + 0.22303938896501776649e-1 * t41155 - 0.39828462315181744017e-2 * t41158 + 0.70806155226989767141e-2 * t41160 - 0.13939961810313610406e-1 * t41162;
    let t41165 = t25640 * t36;
    (t41164, t41165)
}
