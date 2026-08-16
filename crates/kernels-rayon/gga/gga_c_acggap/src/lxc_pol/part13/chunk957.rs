//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 957/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk957(t1089: f64, t31520: f64, t31521: f64, t368: f64, t1198: f64, t2095: f64, t355: f64, t151: f64, t7731: f64, t950: f64, t947: f64, t7685: f64, t932: f64) -> (f64, f64, f64, f64) {
    let t31805 = t31520 * t1089 * t368 * t31521;
    let t31806 = 0.64311027177104605458e-3_f64 * t31805;
    let t31808 = t2095 * t1198 * t355;
    let t31811 = t151 * t7731 * t950;
    let t31812 = t31811 * t947;
    let t31816 = t7685 * t932;
    (t31806, t31808, t31812, t31816)
}
