//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1153/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1153<F: Float>(t14238: F, t14487: F, t360: F, t1021: F, t248: F, t3053: F, t4644: F, t10422: F, t4578: F, t3070: F, t1603: F, t3030: F) -> (F, F, F, F, F) {
    let t14488 = t14238 + t14487;
    let t14489 = t14488 * t360;
    let t14491 = t248 * t1021 * t14489;
    let t14495 = t4644 * t3053 / F::cast_from(3456.0_f64);
    let t14501 = t10422 * t4578;
    let t14503 = t3070 * t14501 / F::cast_from(3456.0_f64);
    let t14506 = t1603 * t3030;
    (t14488, t14491, t14495, t14503, t14506)
}
