//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1861/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1861(t116: f64, t26209: f64, t94973: f64, t26375: f64, t531: f64, t198: f64, t206: f64, t7427: f64, t2411: f64, t26580: f64, t25373: f64, t26550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95357 = t26209 * t116;
    let t95397 = 308.0_f64 / 27.0_f64 * t94973;
    let t95464 = t531 * t26375;
    let t95511 = t198 * t206 * t7427;
    let t95527 = t26580 * t2411;
    let t95536 = t25373 * t26550;
    (t95357, t95397, t95464, t95511, t95527, t95536)
}
