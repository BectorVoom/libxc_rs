//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 778/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk778(t5616: f64, t604: f64, t1181: f64, t2068: f64, t7380: f64, t8544: f64, t2095: f64, t8505: f64, t137: f64, t1579: f64, t336: f64, t578: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8738 = t604 * t5616;
    let t8739 = t1181 * t8738;
    let t8740 = t2068 * t8739;
    let t8742 = t7380 * t8544;
    let t8744 = t2095 * t8505;
    let t8747 = t336 * t1579 * t137;
    let t8748 = t578 * t8747;
    (t8739, t8740, t8742, t8744, t8747, t8748)
}
