//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1880/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1880(t265: f64, t502: f64, t26968: f64, t27032: f64, t3801: f64, t7669: f64, t12587: f64, t2155: f64, t1298: f64, t1300: f64, t198: f64, t25743: f64, t336: f64, t3794: f64, t3798: f64, t5023: f64, t7673: f64) -> (f64, f64, f64, f64) {
    let t503 = t265 < t502;
    let t27033 = t26968 + t27032;
    let t27037 = t7669 * t3801;
    let t27041 = t2155 * t12587;
    let t27048 = piecewise3(t503, t1300 * t198 * t27033 * t336 - 2.0_f64 * t1298 * t27037 * t5023 + 2.0_f64 * t27041 * t3798 * t5023 - t3794 * t5023 * t7673, t25743);
    (t27033, t27037, t27041, t27048)
}
