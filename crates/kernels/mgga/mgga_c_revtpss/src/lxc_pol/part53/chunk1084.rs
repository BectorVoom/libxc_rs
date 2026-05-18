//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1084/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1084<F: Float>(t119763: F, t839: F, t31752: F, t31753: F, t854: F, t27: F, t2718: F, t8484: F, t25386: F, t2487: F, t826: F, t231: F, t886: F) -> (F, F, F, F, F, F, F) {
    let t119764 = t119763 * t839;
    let t119767 = t31752 * t31753 * t854;
    let t119776 = t8484 * t2718 * t27;
    let t119777 = t25386 * t119776;
    let t119778 = t119777 * t2487;
    let t119781 = t31752 * t31753 * t826;
    let t119783 = t231 * t886;
    (t119764, t119767, t119776, t119777, t119778, t119781, t119783)
}
