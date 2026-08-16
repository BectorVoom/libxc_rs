//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 866/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk866(t30105: f64, t7365: f64, t2067: f64, t4180: f64, t3427: f64, t7647: f64, t1530: f64, t7584: f64, t129: f64, t361: f64, t3360: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30106 = t30105 * t7365;
    let t30120 = t4180 * t2067;
    let t30123 = t7647 * t3427;
    let t30127 = t1530 * t7584;
    let t30137 = t129 * t361;
    let t30147 = t3360 * t7584;
    (t30106, t30120, t30123, t30127, t30137, t30147)
}
