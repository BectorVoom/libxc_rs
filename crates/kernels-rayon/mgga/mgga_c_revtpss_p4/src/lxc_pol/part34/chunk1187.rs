//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1187/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1187(t1513: f64, t5915: f64, t116: f64, t22746: f64, t14586: f64, t6016: f64, t1558: f64, t231: f64, t221: f64, t23279: f64, t22648: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75833 = t1513 * t5915;
    let t75941 = t22746 * t116;
    let t76106 = t14586 * t6016;
    let t76161 = t6016 * t1558 * t231;
    let t76613 = t221 * t23279;
    let t85037 = t22648 * t602;
    (t75833, t75941, t76106, t76161, t76613, t85037)
}
