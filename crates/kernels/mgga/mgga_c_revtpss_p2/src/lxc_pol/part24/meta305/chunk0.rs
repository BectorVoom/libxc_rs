//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1090/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1090<F: Float>(t1469: F, t70: F, t72: F, t10355: F, t5819: F, t10368: F, t10389: F, t10398: F, t5892: F, t625: F, t10208: F, t5891: F) -> (F, F, F, F, F, F, F) {
    let t21686 = t1469 * t70 * t72;
    let t21732 = t10355 * t5819;
    let t21754 = t10368 * t5819;
    let t21784 = t10389 * t5819;
    let t21794 = t10398 * t5819;
    let t21818 = t625 * t5892;
    let t21820 = t10208 * t5891;
    (t21686, t21732, t21754, t21784, t21794, t21818, t21820)
}
