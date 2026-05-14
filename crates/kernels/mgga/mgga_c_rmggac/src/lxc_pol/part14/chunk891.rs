//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 891/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk891<F: Float>(t5204: F, t649: F, t8764: F, t5207: F, t5211: F, t7599: F, t5199: F, t5187: F, t5218: F, t5194: F, t8746: F, t41055: F, t851: F, t36063: F, t36065: F, t36072: F, t36074: F, t36078: F, t36088: F, t36090: F, t36092: F) -> (F, F, F, F, F, F, F, F) {
    let t41209 = t649 * t5204;
    let t41210 = t8764 * t41209;
    let t41212 = t649 * t5207;
    let t41213 = t8764 * t41212;
    let t41215 = t649 * t5211;
    let t41216 = t7599 * t41215;
    let t41218 = t649 * t5199;
    let t41219 = t7599 * t41218;
    let t41221 = t649 * t5187;
    let t41222 = t7599 * t41221;
    let t41224 = t649 * t5218;
    let t41225 = t7599 * t41224;
    let t41227 = t649 * t5194;
    let t41228 = t8746 * t41227;
    let t41230 = t851 * t41055;
    let t41231 = 0.17701538806747441785e-2 * t41230;
    let t41232 = -0.22579296166828959078e-2 * t36063 - 0.60610359231656314955e-1 * t36065 - 0.1209605151794408522e-2 * t36072 - 0.18183107769496894486e-1 * t36074 + 0.12122071846331262991e-1 * t36078 + 0.1774087555965132499e-2 * t36088 - 0.20697688152926545822e-2 * t36090 - 0.2419210303588817044e-3 * t36092 + 0.13637330827122670865e-1 * t41210 + 0.68186654135613354324e-2 * t41213 - 0.2727466165424534173e-1 * t41216 - 0.13637330827122670865e-1 * t41219 - 0.2727466165424534173e-1 * t41222 - 0.13637330827122670865e-1 * t41225 + 0.45457769423742236216e-1 * t41228 + t41231;
    (t41209, t41212, t41215, t41218, t41221, t41224, t41227, t41232)
}
