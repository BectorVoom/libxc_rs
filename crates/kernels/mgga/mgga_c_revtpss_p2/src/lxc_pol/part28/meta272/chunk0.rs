//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1218/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1218<F: Float>(t1470: F, t603: F, t1469: F, t6968: F, t6971: F, t72: F, t1927: F) -> (F, F, F, F) {
    let t7709 = t603 * t1470;
    let t7714 = F::new(5.0) / F::new(6.0) * t6968 * t1469 + t6971;
    let t7715 = t7714 * t72;
    let t7716 = t7715 * t1927;
    (t7709, t7714, t7715, t7716)
}
