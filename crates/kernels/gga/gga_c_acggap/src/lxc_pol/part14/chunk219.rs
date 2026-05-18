//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 219/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk219<F: Float>(t219: F, t771: F, t201: F, t199: F, t13: F, t30: F, t761: F, t132: F, t265: F, t264: F, t80: F, t75: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t772 = t771 * t219;
    let t773 = t201 * t772;
    let t774 = F::new(1.0) * t773;
    let t775 = t199 * t199;
    let t776 = F::new(1.0) / t775;
    let t777 = t13 * t776;
    let t778 = t30 * t30;
    let t779 = F::new(1.0) / t778;
    let t780 = t761 * t779;
    let t781 = t777 * t780;
    let t782 = F::new(0.16081979498692535067e2) * t781;
    let t786 = t132 * t265;
    let t790 = t264 * t80;
    let t791 = F::new(1.0) / t790;
    let t792 = t75 * t791;
    (t772, t774, t775, t776, t777, t778, t779, t780, t782, t786, t791, t792)
}
