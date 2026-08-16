//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 885/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk885(t11328: f64, t11343: f64, t1137: f64, t1127: f64, t3355: f64, t427: f64, t3358: f64, t435: f64, t11306: f64, t1147: f64, t3368: f64, t1143: f64, t3400: f64) -> (f64, f64, f64, f64, f64) {
    let t11344 = t11328 + t11343;
    let t11345 = t11344 * t1137;
    let t11349 = 1.0_f64 / t3355 / t1127;
    let t11350 = t427 * t11349;
    let t11352 = 1.0_f64 / t3358 / t435;
    let t11353 = t11306 * t11352;
    let t11356 = t3368 * t1147;
    let t11361 = t1143 * t3400;
    (t11345, t11350, t11353, t11356, t11361)
}
