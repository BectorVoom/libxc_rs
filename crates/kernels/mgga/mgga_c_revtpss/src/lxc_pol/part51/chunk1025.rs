//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1025/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1025<F: Float>(t119821: F, t822: F, t31830: F, t122: F, t72: F, t8471: F, t2466: F, t25377: F, t676: F, t7048: F, t32474: F, t1032: F, t7063: F) -> (F, F, F, F, F, F, F, F) {
    let t119822 = t119821 * t822;
    let t119823 = t31830 * t119822;
    let t119825 = t8471 * t72 * t122;
    let t119826 = t119825 * t2466;
    let t119827 = t119823 * t119826;
    let t119830 = t25377 * t676 * t7048;
    let t119831 = t32474 * t119830;
    let t119833 = t7063 * t1032;
    (t119822, t119823, t119825, t119826, t119827, t119830, t119831, t119833)
}
