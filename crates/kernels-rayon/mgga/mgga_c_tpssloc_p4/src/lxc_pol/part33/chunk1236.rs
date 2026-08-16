//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1236/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1236(t82218: f64, t10109: f64, t225: f64, t1914: f64, t40772: f64, t1054: f64, t2775: f64, t10213: f64, t344: f64, t381: f64, t2770: f64, t10189: f64, t1926: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t82219 = 0.27720185200590482541e0_f64 * t82218;
    let t82252 = t225 * t10109;
    let t82312 = t1914 * t40772;
    let t82342 = t1054 * t2775;
    let t82390 = t10213 * t344;
    let t82391 = t82390 * t381;
    let t82411 = t1054 * t2770;
    let t82431 = t1926 * t221 * t10189;
    (t82219, t82252, t82312, t82342, t82390, t82391, t82411, t82431)
}
