//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 983/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk983(t1128: f64, t3324: f64, t1124: f64, t3356: f64, t3355: f64, t432: f64, t427: f64, t1094: f64, t3263: f64, t3395: f64, t3403: f64, t11135: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11410 = t3324 * t1128;
    let t11415 = t1124 * t3356;
    let t11419 = 1.0_f64 / t3355 / t432;
    let t11420 = t427 * t11419;
    let t11424 = t1094 * t3263;
    let t11433 = t3395 * t3403;
    let t11444 = 0.53272592592592592592e-1_f64 * t11135;
    (t11410, t11415, t11420, t11424, t11433, t11444)
}
