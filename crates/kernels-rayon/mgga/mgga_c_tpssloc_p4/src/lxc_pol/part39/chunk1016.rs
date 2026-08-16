//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1016/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1016(t25: f64, t28: f64, t4021: f64, t645: f64, t1437: f64, t2307: f64, t1409: f64, t9321: f64, t2291: f64, t3966: f64, t584: f64, t9212: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t12585 = t4021 * t645;
    let t12588 = t1437 * t2307;
    let t12595 = t9321 * t1409;
    let t12598 = t2291 * t3966;
    let t12603 = 2.0_f64 * t584;
    let t12604 = 6.0_f64 * t9212;
    let t12606 = piecewise5(t26, 0.0_f64, t29, 0.0_f64, t12603 - t12604);
    (t12585, t12588, t12595, t12598, t12606)
}
