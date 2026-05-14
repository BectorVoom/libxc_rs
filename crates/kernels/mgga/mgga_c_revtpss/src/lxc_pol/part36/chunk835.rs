//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 835/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk835<F: Float>(t372: F, t5277: F, t1778: F, t3682: F, t1770: F, t3766: F, t3754: F, t5219: F, t1811: F, t3566: F, t17306: F, t487: F, t116: F, t5876: F) -> (F, F, F, F, F, F, F) {
    let t17661 = t372 * t5277;
    let t17792 = t1778 * t3682;
    let t17934 = t1770 * t3766;
    let t17958 = t5219 * t3754;
    let t17995 = t3566 * t1811;
    let t18059 = t17306 * t487;
    let t18245 = t5876 * t116;
    (t17661, t17792, t17934, t17958, t17995, t18059, t18245)
}
