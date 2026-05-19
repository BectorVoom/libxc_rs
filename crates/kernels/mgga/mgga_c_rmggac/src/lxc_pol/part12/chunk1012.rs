//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1012/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1012<F: Float>(t5194: F, t649: F, t8746: F, t41055: F, t851: F, t36063: F, t36065: F, t36072: F, t36074: F, t36078: F, t36088: F, t36090: F, t36092: F, t41210: F, t41213: F, t41216: F, t41219: F, t41222: F, t41225: F) -> (F, F) {
    let t41227 = t649 * t5194;
    let t41228 = t8746 * t41227;
    let t41230 = t851 * t41055;
    let t41231 = F::cast_from(0.17701538806747441785e-2_f64) * t41230;
    let t41232 = -F::cast_from(0.22579296166828959078e-2_f64) * t36063 - F::cast_from(0.60610359231656314955e-1_f64) * t36065 - F::cast_from(0.1209605151794408522e-2_f64) * t36072 - F::cast_from(0.18183107769496894486e-1_f64) * t36074 + F::cast_from(0.12122071846331262991e-1_f64) * t36078 + F::cast_from(0.1774087555965132499e-2_f64) * t36088 - F::cast_from(0.20697688152926545822e-2_f64) * t36090 - F::cast_from(0.2419210303588817044e-3_f64) * t36092 + F::cast_from(0.13637330827122670865e-1_f64) * t41210 + F::cast_from(0.68186654135613354324e-2_f64) * t41213 - F::cast_from(0.2727466165424534173e-1_f64) * t41216 - F::cast_from(0.13637330827122670865e-1_f64) * t41219 - F::cast_from(0.2727466165424534173e-1_f64) * t41222 - F::cast_from(0.13637330827122670865e-1_f64) * t41225 + F::cast_from(0.45457769423742236216e-1_f64) * t41228 + t41231;
    (t41227, t41232)
}
