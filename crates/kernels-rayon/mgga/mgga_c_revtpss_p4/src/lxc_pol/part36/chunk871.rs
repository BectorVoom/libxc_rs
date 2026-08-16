//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 871/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk871(t2777: f64, t5759: f64, t2439: f64, t136: f64, t1883: f64, t2457: f64, t10139: f64, t1892: f64, t4086: f64, t786: f64, t2470: f64, t5740: f64) -> (f64, f64, f64, f64) {
    let t14202 = t2777 * t5759;
    let t14203 = t2439 * t14202;
    let t14219 = t1883 * t136;
    let t14220 = t14219 * t2457;
    let t14221 = t10139 * t14220;
    let t14238 = t4086 * t1892;
    let t14239 = t786 * t14238;
    let t14242 = t5740 * t2470;
    (t14203, t14221, t14239, t14242)
}
