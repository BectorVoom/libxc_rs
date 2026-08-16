//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1502/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1502(t2212: f64, t6936: f64, t118089: f64, t118091: f64, t118094: f64, t118099: f64, t118106: f64, t118629: f64, t118957: f64, t118962: f64, t1456: f64, t1458: f64, t1464: f64, t1914: f64, t2205: f64, t22571: f64, t3: f64, t31512: f64, t31701: f64, t31737: f64, t575: f64, t5808: f64, t8417: f64) -> f64 {
    let t118968 = t6936 * t2212;
    let t118975 = t1458 * (t118629 + t118962) + t118089 + t118091 + t118094 + t3 * t118957 * t575 + t2205 * t22571 + t118968 + t1456 * t31737 + t31701 * t1464 + t118099 + 2.0_f64 * t1914 * t31512 + 2.0_f64 * t8417 * t5808 + t118106;
    t118975
}
