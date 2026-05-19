//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 989/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk989<F: Float>(t2441: F, t9303: F, t10115: F, t258: F, t2453: F, t2464: F, t2438: F, t886: F, t138: F, t2434: F, t123: F, t2465: F) -> (F, F, F, F, F) {
    let t10501 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t2441;
    let t10503 = F::cast_from(0.11044544084478153697e-3_f64) * t10115 * t258;
    let t10504 = t2453 * t2464;
    let t10505 = t2438 * t886;
    let t10506 = t138 * t10505;
    let t10507 = t10504 * t10506;
    let t10509 = t2434 * t886;
    let t10510 = t123 * t10509;
    let t10511 = t2465 * t10510;
    (t10501, t10503, t10504, t10507, t10511)
}
