//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 962/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk962(t1198: f64, t2095: f64, t355: f64, t151: f64, t7731: f64, t950: f64, t947: f64, t7685: f64, t932: f64, t2016: f64, t7592: f64, t3378: f64, t7560: f64) -> (f64, f64, f64, f64, f64) {
    let t31808 = t2095 * t1198 * t355;
    let t31811 = t151 * t7731 * t950;
    let t31812 = t31811 * t947;
    let t31816 = t7685 * t932;
    let t31822 = t2016 * t7592;
    let t31824 = t3378 * t7560;
    (t31808, t31812, t31816, t31822, t31824)
}
