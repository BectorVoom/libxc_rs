//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 436/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk436<F: Float>(t2694: F, t807: F, t21: F, t65: F, t64: F, t159: F, t222: F, t794: F, t798: F, t802: F, t234: F, t2453: F, t595: F, t235: F, t826: F, t232: F, t821: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2695 = t807 * t2694;
    let t2698 = 1.0 / t65 / t21;
    let t2699 = t64 * t2698;
    let t2700 = t2699 * t159;
    let t2702 = 35.0 / 432.0 * t2700 * t222;
    let t2703 = t794 * t798;
    let t2704 = t2703 * t802;
    let t2710 = t2453 * t234;
    let t2712 = 1.0 / t65 / t595;
    let t2713 = t235 * t2712;
    let t2716 = 0.45178982497454656791e-5 * t2710 * t2713 * t826;
    let t2718 = 1.0 / t821 / t232;
    (t2695, t2698, t2700, t2702, t2703, t2704, t2710, t2712, t2713, t2716, t2718)
}
