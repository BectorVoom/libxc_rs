//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 904/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk904<F: Float>(t18738: F, t2782: F, t18677: F, t231: F, t2783: F, t18681: F, t2723: F, t4503: F, t6041: F, t72: F, t686: F, t874: F) -> (F, F, F, F, F) {
    let t18739 = t2782 * t18738;
    let t18742 = t2783 * t18677 * t231;
    let t18743 = t2782 * t18742;
    let t18746 = t2783 * t18681 * t231;
    let t18747 = t2782 * t18746;
    let t18750 = t4503 * t18677 * t2723;
    let t18751 = t2782 * t18750;
    let t18761 = t6041 * t72;
    let t18763 = t874 * t18761 * t686;
    (t18739, t18743, t18747, t18751, t18763)
}
