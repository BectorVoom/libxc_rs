//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1587/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1587<F: Float>(t5816: F, t644: F, t1497: F, t4241: F, t5872: F, t1469: F, t70: F, t72: F, t1927: F, t4186: F, t5819: F, t627: F) -> (F, F, F, F, F, F) {
    let t21674 = t5816 * t644;
    let t21677 = t1497 * t4241;
    let t21682 = t5872 * t644;
    let t21686 = t1469 * t70 * t72;
    let t21687 = t1927 * t4186;
    let t21690 = t5819 * t627;
    (t21674, t21677, t21682, t21686, t21687, t21690)
}
