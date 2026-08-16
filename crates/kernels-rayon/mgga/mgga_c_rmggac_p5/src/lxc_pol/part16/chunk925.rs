//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 925/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk925(t2281: f64, t34975: f64, t35039: f64, t9145: f64, t16503: f64, t38508: f64, t8420: f64, t1859: f64, t1979: f64, t1982: f64, t201: f64, t446: f64) -> (f64, f64, f64) {
    let t45385 = t34975 * t35039 * t2281 * t9145;
    let t45389 = t16503 * t38508 * t2281 * t8420;
    let t45394 = t446 * t1859 * t201 * t1979 * t1982;
    (t45385, t45389, t45394)
}
