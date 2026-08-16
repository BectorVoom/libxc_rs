//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1146/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1146(t127143: f64, t127180: f64, t27375: f64, t27799: f64, t125984: f64, t25759: f64, t126030: f64, t100981: f64, t27384: f64, t1113: f64, t7782: f64, t1711: f64, t7086: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127181 = t127143 + t127180;
    let t127190 = t27799 * t27375;
    let t127193 = t25759 * t125984;
    let t127199 = t25759 * t126030;
    let t127204 = t100981 * t27384;
    let t127207 = t1113 * t7782;
    let t127212 = t1711 * t7086;
    (t127181, t127190, t127193, t127199, t127204, t127207, t127212)
}
