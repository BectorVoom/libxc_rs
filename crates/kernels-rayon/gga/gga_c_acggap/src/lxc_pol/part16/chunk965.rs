//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 965/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk965(t34217: f64, t1988: f64, t8566: f64, t1181: f64, t4521: f64, t604: f64, t7426: f64, t1466: f64, t30644: f64, t137: f64, t14423: f64, t30209: f64, t5099: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34218 = 0.62896184579208304136e-3_f64 * t34217;
    let t34221 = t1988 * t8566;
    let t34222 = 0.62896184579208304136e-3_f64 * t34221;
    let t34237 = t7426 * t1181 * t604 * t4521;
    let t34239 = t30644 * t1466;
    let t34240 = 0.17149607247227894789e-2_f64 * t34239;
    let t34248 = t14423 * t137;
    let t34255 = t30209 * t1181 * t604 * t5099;
    (t34218, t34222, t34237, t34240, t34248, t34255)
}
