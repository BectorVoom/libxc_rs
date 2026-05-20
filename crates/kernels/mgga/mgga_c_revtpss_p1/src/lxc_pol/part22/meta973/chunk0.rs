//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3260/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3260<F: Float>(t18413: F, t18525: F, t2661: F, t40693: F, t10726: F, t4366: F, t2723: F, t61647: F, t10886: F, t18608: F, t808: F, t2394: F, t2721: F, t40462: F, t40625: F, t40630: F, t40638: F, t40639: F, t40645: F, t40654: F, t5966: F, t827: F, t828: F, t851: F) -> (F, F) {
    let t61860 = t2661 * t40693 * t18413 * t18525;
    let t61864 = t2661 * t10726 * t18413 * t4366;
    let t61866 = t61647 * t2723;
    let t61877 = t10886 * t808 * t18608;
    let t61879 = F::cast_from(0.90357964994909313584e-6_f64) * t40625 + F::cast_from(0.36143185997963725432e-4_f64) * t40630 - t40638 + F::cast_from(0.57800528129545867622e-2_f64) * t40639 - F::cast_from(0.30488190661738479624e-3_f64) * t40645 + t40654 + F::cast_from(0.85748036236139473945e-4_f64) * t61860 - F::cast_from(0.85748036236139473945e-4_f64) * t61864 + F::cast_from(0.85748036236139473944e-3_f64) * t2721 * t827 * t828 * t61866 + F::cast_from(0.18007087609589289528e0_f64) * t851 * t40462 * t828 * t5966 * t2394 + F::cast_from(0.10164000561857065645e-4_f64) * t61877;
    (t61866, t61879)
}
