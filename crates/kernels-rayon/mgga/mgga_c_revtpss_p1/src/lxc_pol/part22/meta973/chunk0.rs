//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3260/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3260(t18413: f64, t18525: f64, t2661: f64, t40693: f64, t10726: f64, t4366: f64, t2723: f64, t61647: f64, t10886: f64, t18608: f64, t808: f64, t2394: f64, t2721: f64, t40462: f64, t40625: f64, t40630: f64, t40638: f64, t40639: f64, t40645: f64, t40654: f64, t5966: f64, t827: f64, t828: f64, t851: f64) -> (f64, f64) {
    let t61860 = t2661 * t40693 * t18413 * t18525;
    let t61864 = t2661 * t10726 * t18413 * t4366;
    let t61866 = t61647 * t2723;
    let t61877 = t10886 * t808 * t18608;
    let t61879 = 0.90357964994909313584e-6_f64 * t40625 + 0.36143185997963725432e-4_f64 * t40630 - t40638 + 0.57800528129545867622e-2_f64 * t40639 - 0.30488190661738479624e-3_f64 * t40645 + t40654 + 0.85748036236139473945e-4_f64 * t61860 - 0.85748036236139473945e-4_f64 * t61864 + 0.85748036236139473944e-3_f64 * t2721 * t827 * t828 * t61866 + 0.18007087609589289528e0_f64 * t851 * t40462 * t828 * t5966 * t2394 + 0.10164000561857065645e-4_f64 * t61877;
    (t61866, t61879)
}
