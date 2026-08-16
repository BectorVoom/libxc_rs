//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 666/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk666(t25: f64, t28: f64, t1298: f64, t3704: f64, t5397: f64, t6305: f64, t1302: f64, t3711: f64, t5966: f64, t6312: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t6339 = piecewise3(t26, 0.0_f64, -2.0_f64 / 9.0_f64 * t3704 * t6305 + 2.0_f64 / 3.0_f64 * t1298 * t5397);
    let t6345 = piecewise3(t29, 0.0_f64, -2.0_f64 / 9.0_f64 * t3711 * t6312 + 2.0_f64 / 3.0_f64 * t1302 * t5966);
    let t6347 = t6339 / 2.0_f64 + t6345 / 2.0_f64;
    t6347
}
