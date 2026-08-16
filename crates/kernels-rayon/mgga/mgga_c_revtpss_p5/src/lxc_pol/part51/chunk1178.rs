//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1178/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1178(t125984: f64, t25759: f64, t126030: f64, t100981: f64, t27384: f64, t1113: f64, t7782: f64, t1711: f64, t7086: f64, t125961: f64, t27799: f64, t27363: f64, t33: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127193 = t25759 * t125984;
    let t127199 = t25759 * t126030;
    let t127204 = t100981 * t27384;
    let t127207 = t1113 * t7782;
    let t127212 = t1711 * t7086;
    let t127218 = t27799 * t125961;
    let t127227 = t33 * t27363;
    (t127193, t127199, t127204, t127207, t127212, t127218, t127227)
}
