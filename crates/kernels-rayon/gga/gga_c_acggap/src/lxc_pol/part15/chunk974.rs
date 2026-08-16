//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 974/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk974(t7440: f64, t8631: f64, t2318: f64, t31261: f64, t7538: f64, t8689: f64, t1352: f64, t7746: f64, t1967: f64, t8486: f64, t7736: f64, t2450: f64, t31349: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34390 = t7440 * t8631;
    let t34392 = t31261 * t2318;
    let t34394 = t7538 * t8689;
    let t34396 = t7746 * t1352;
    let t34398 = t1967 * t8486;
    let t34400 = t7736 * t1352;
    let t34406 = t2450 * t31349;
    (t34390, t34392, t34394, t34396, t34398, t34400, t34406)
}
