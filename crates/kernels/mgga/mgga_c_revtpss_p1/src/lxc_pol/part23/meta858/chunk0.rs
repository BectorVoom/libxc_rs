//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2748/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2748<F: Float>(t3603: F, t43350: F, t13126: F, t1811: F, t460: F, t3566: F, t6695: F, t5216: F, t17288: F, t488: F, t5219: F, t487: F, t69636: F) -> (F, F, F, F, F, F, F) {
    let t72724 = t43350 * t3603;
    let t72732 = t460 * t13126 * t1811;
    let t72767 = t3566 * t6695;
    let t72784 = t5216 * t1811;
    let t72787 = t17288 * t1811;
    let t72794 = t5219 * t488;
    let t72802 = t69636 * t487;
    (t72724, t72732, t72767, t72784, t72787, t72794, t72802)
}
