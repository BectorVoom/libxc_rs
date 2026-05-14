//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 750/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk750<F: Float>(t10494: F, t2770: F, t2435: F, t2445: F, t2441: F, t9303: F, t10115: F, t258: F, t2453: F, t2464: F, t2438: F, t886: F, t138: F, t2434: F, t123: F, t2465: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10495 = t2770 * t10494;
    let t10498 = t2435 * t2445;
    let t10501 = 0.26019841438354088051e-2 * t9303 * t2441;
    let t10503 = 0.11044544084478153697e-3 * t10115 * t258;
    let t10504 = t2453 * t2464;
    let t10505 = t2438 * t886;
    let t10506 = t138 * t10505;
    let t10507 = t10504 * t10506;
    let t10509 = t2434 * t886;
    let t10510 = t123 * t10509;
    let t10511 = t2465 * t10510;
    (t10495, t10498, t10501, t10503, t10505, t10506, t10507, t10509, t10510, t10511)
}
