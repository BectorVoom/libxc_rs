//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1481/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1481(t21881: f64, t93: f64, t10208: f64, t625: f64, t46157: f64, t69: f64, t2289: f64, t2339: f64, t8260: f64, t655: f64, t8269: f64, t2349: f64, t43: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t109242 = t93 * t21881;
    let t116912 = t625 * t10208;
    let t116919 = t69 * t46157;
    let t116926 = t2289 * t2339;
    let t116927 = t116926 * t8260;
    let t116929 = t2289 * t655;
    let t116930 = t116929 * t8269;
    let t116942 = t43 * t2349;
    (t109242, t116912, t116919, t116926, t116927, t116929, t116930, t116942)
}
