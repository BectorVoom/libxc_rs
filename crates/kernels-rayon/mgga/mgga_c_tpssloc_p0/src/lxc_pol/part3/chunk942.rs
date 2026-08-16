//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 942/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk942(t11349: f64, t427: f64, t3358: f64, t435: f64, t1147: f64, t3368: f64, t1143: f64, t3400: f64, t11292: f64, t440: f64, t11135: f64, t11203: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11350 = t427 * t11349;
    let t11352 = 1.0_f64 / t3358 / t435;
    let t11356 = t3368 * t1147;
    let t11361 = t1143 * t3400;
    let t11365 = t440 * t11292;
    let t11369 = 0.93932222222222222223e0_f64 * t11135;
    let t11372 = 0.36793333333333333333e0_f64 * t11203;
    (t11350, t11352, t11356, t11361, t11365, t11369, t11372)
}
