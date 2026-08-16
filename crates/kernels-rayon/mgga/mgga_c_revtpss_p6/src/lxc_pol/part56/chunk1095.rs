//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1095/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1095(t120956: f64, t125587: f64, t1414: f64, t828: f64, t121090: f64, t27888: f64, t121093: f64, t121019: f64, t32284: f64, t5700: f64, t121018: f64, t1399: f64, t33962: f64) -> (f64, f64, f64, f64, f64) {
    let t125590 = t120956 * t1414 * t828 * t125587;
    let t125594 = t121090 * t27888;
    let t125596 = t121093 * t27888;
    let t125599 = t32284 * t121019 * t5700;
    let t125603 = t121018 * t121019 * t33962 * t1399;
    (t125590, t125594, t125596, t125599, t125603)
}
