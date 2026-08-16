//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1535/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1535(t2857: f64, t3154: f64, t2251: f64, t2258: f64, t10326: f64, t606: f64, t11262: f64, t3127: f64, t3129: f64, t11240: f64, t11628: f64, t42646: f64) -> (f64, f64, f64, f64, f64) {
    let t43174 = t3154 * t2857;
    let t43175 = t43174 * t2251;
    let t43180 = t2251 * t2258;
    let t43194 = t10326 * t606;
    let t43204 = t3127 * t11262 * t3129;
    let t43207 = t11240 * t11628 * t42646;
    (t43175, t43180, t43194, t43204, t43207)
}
