//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 806/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk806<F: Float>(t2441: F, t9303: F, t10115: F, t258: F, t2453: F, t2464: F, t251: F, t4503: F, t786: F, t2797: F, t760: F, t9323: F) -> (F, F, F, F, F, F) {
    let t10501 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t2441;
    let t10503 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t258;
    let t10504 = t2453 * t2464;
    let t10529 = t4503 * t251;
    let t10530 = t786 * t10529;
    let t10535 = t2453 * t2797;
    let t10552 = F::cast_from(0.51947577317044391277e2_f64) * t760 * t9323;
    (t10501, t10503, t10504, t10530, t10535, t10552)
}
