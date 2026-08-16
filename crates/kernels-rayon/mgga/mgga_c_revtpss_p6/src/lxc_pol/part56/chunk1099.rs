//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1099/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1099(t125648: f64, t1381: f64, t8590: f64, t121181: f64, t5741: f64, t121146: f64, t32195: f64, t32206: f64, t3936: f64, t5591: f64, t121204: f64, t1868: f64, t9818: f64) -> (f64, f64, f64, f64, f64) {
    let t125650 = t125648 * t8590 * t1381;
    let t125652 = t121181 * t5741;
    let t125654 = t121146 * t5741;
    let t125659 = t32206 * t3936 * t32195 * t5591;
    let t125662 = t9818 * t121204 * t1868;
    (t125650, t125652, t125654, t125659, t125662)
}
