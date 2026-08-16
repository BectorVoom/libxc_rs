//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 924/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk924(t1089: f64, t31520: f64, t31521: f64, t368: f64, t151: f64, t7731: f64, t950: f64, t3378: f64, t7560: f64, t30049: f64, t7461: f64, t2104: f64, t7610: f64) -> (f64, f64, f64, f64, f64) {
    let t31805 = t31520 * t1089 * t368 * t31521;
    let t31806 = 0.64311027177104605458e-3_f64 * t31805;
    let t31811 = t151 * t7731 * t950;
    let t31824 = t3378 * t7560;
    let t31839 = t30049 * t7461;
    let t31840 = 0.42874018118069736972e-3_f64 * t31839;
    let t31849 = t7610 * t2104;
    (t31806, t31811, t31824, t31840, t31849)
}
