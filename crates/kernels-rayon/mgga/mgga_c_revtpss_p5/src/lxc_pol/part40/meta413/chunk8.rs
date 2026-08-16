//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1502/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1502(t117168: f64, t117170: f64, t118085: f64, t118089: f64, t118091: f64, t118094: f64, t118099: f64, t1464: f64, t18178: f64, t18217: f64, t1921: f64, t2205: f64, t2212: f64, t3: f64, t31205: f64, t31464: f64, t4168: f64, t575: f64, t5808: f64, t8331: f64, t8417: f64) -> f64 {
    let t118100 = t118085 * t3 * t575 + 2.0_f64 * t1464 * t31464 + t18178 * t2212 + t18217 * t2205 + t1921 * t31205 + t4168 * t8417 + 2.0_f64 * t5808 * t8331 + t117168 + t117170 + t118089 + t118091 + t118094 + t118099;
    t118100
}
