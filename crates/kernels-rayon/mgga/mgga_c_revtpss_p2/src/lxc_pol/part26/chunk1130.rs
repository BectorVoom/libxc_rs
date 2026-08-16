//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1130/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1130(t2434: f64, t837: f64, t251: f64, t25304: f64, t25374: f64, t68: f64, t785: f64, t281: f64, t10910: f64, t1955: f64, t231: f64, t2645: f64, t886: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93182 = t2434 * t837;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    let t93238 = t68 * t785;
    let t93240 = t281 * t93238 * t251;
    let t93244 = t1955 * t10910;
    let t93267 = t886 * t2645 * t231;
    (t93182, t93189, t93190, t93238, t93240, t93244, t93267)
}
