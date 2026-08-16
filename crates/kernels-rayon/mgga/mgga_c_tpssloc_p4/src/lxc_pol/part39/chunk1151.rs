//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1151/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1151(t14238: f64, t14487: f64, t360: f64, t1021: f64, t248: f64, t3053: f64, t4644: f64, t10422: f64, t4578: f64, t3070: f64, t1603: f64, t3030: f64) -> (f64, f64, f64, f64, f64) {
    let t14488 = t14238 + t14487;
    let t14489 = t14488 * t360;
    let t14491 = t248 * t1021 * t14489;
    let t14495 = t4644 * t3053 / 3456.0_f64;
    let t14501 = t10422 * t4578;
    let t14503 = t3070 * t14501 / 3456.0_f64;
    let t14506 = t1603 * t3030;
    (t14488, t14491, t14495, t14503, t14506)
}
