//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2094/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2094<F: Float>(t27381: F, t7294: F, t24574: F, t27383: F, t7288: F, t94490: F, t27438: F, t85639: F, t225: F, t27419: F, t27427: F, t5052: F, t7284: F) -> (F, F, F, F, F, F, F) {
    let t94584 = t7294 * t27381;
    let t94628 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27383;
    let t94631 = t94490 * t7288;
    let t94648 = F::cast_from(0.18277045187202515961e-2_f64) * t85639 * t27438;
    let t94656 = t27419 * t225;
    let t94676 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27427;
    let t94680 = t7284 * t5052;
    (t94584, t94628, t94631, t94648, t94656, t94676, t94680)
}
