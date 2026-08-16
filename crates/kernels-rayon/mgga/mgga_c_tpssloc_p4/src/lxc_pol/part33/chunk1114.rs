//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1114/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1114(t2978: f64, t344: f64, t381: f64, t3034: f64, t38: f64, t131: f64, t350: f64) -> (f64, f64, f64, f64, f64) {
    let t23592 = t2978 * t344;
    let t23593 = t23592 * t381;
    let t23598 = 1.0_f64 / t3034;
    let t23599 = t38 * t23598;
    let t23600 = t23599 * t131;
    let t23601 = t23600 * t350;
    (t23592, t23593, t23599, t23600, t23601)
}
