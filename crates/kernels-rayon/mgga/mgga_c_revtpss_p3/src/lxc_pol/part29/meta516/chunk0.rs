//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1838/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1838(t25410: f64, t93169: f64, t2438: f64, t837: f64, t786: f64, t92889: f64, t2434: f64, t251: f64, t25304: f64, t25374: f64, t68: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93170 = t93169 * t25410;
    let t93173 = t2438 * t837;
    let t93179 = t786 * t92889;
    let t93182 = t2434 * t837;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    let t93238 = t68 * t785;
    (t93170, t93173, t93179, t93182, t93189, t93190, t93238)
}
