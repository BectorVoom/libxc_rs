//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1199/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1199(t25410: f64, t93189: f64, t25374: f64, t93169: f64, t25604: f64, t995: f64, t378: f64, t42859: f64, t1078: f64, t1982: f64, t25610: f64, t3058: f64, t8521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93371 = t93189 * t25410;
    let t93377 = t93169 * t25374;
    let t93436 = t995 * t25604;
    let t93469 = t378 * t42859;
    let t93471 = t1982 * t93469 * t1078;
    let t93497 = t25610 * t25604;
    let t93502 = t3058 * t8521;
    (t93371, t93377, t93436, t93471, t93497, t93502)
}
