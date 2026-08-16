//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 985/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk985(t3355: f64, t432: f64, t427: f64, t11306: f64, t3359: f64, t1094: f64, t3263: f64, t3266: f64, t1118: f64, t11191: f64, t3313: f64, t1157: f64, t3395: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11419 = 1.0_f64 / t3355 / t432;
    let t11420 = t427 * t11419;
    let t11421 = t11306 * t3359;
    let t11424 = t1094 * t3263;
    let t11426 = 6.0_f64 * t11424 * t3266;
    let t11427 = t11191 * t1118;
    let t11429 = 6.0_f64 * t3313 * t11427;
    let t11430 = t1157 * t3395;
    (t11419, t11420, t11421, t11424, t11426, t11427, t11429, t11430)
}
