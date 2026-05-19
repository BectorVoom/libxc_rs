//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1140/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1140<F: Float>(t2013: F, t28166: F, t531: F, t7933: F, t8995: F, t2033: F, t9593: F, t116: F, t7741: F, t27240: F, t27246: F, t27251: F) -> (F, F, F, F, F, F, F, F) {
    let t28167 = t2013 * t28166;
    let t28172 = t531 * t7933;
    let t28196 = t2013 * t8995;
    let t28197 = t2033 * t9593;
    let t28276 = t116 * t7741;
    let t28330 = F::cast_from(0.11433071498151929859e-3_f64) * t27240;
    let t28333 = F::new(7.0) / F::new(72.0) * t27246;
    let t28335 = F::cast_from(0.2032800112371413129e-3_f64) * t27251;
    (t28167, t28172, t28196, t28197, t28276, t28330, t28333, t28335)
}
