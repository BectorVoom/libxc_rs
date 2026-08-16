//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2094/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2094(t27381: f64, t7294: f64, t24574: f64, t27383: f64, t7288: f64, t94490: f64, t27438: f64, t85639: f64, t225: f64, t27419: f64, t27427: f64, t5052: f64, t7284: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94584 = t7294 * t27381;
    let t94628 = 0.54831135561607547884e-2_f64 * t24574 * t27383;
    let t94631 = t94490 * t7288;
    let t94648 = 0.18277045187202515961e-2_f64 * t85639 * t27438;
    let t94656 = t27419 * t225;
    let t94676 = 0.18277045187202515961e-2_f64 * t24574 * t27427;
    let t94680 = t7284 * t5052;
    (t94584, t94628, t94631, t94648, t94656, t94676, t94680)
}
