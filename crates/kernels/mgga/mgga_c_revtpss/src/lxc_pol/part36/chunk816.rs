//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 816/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk816<F: Float>(t136: F, t550: F, t124: F, t1882: F, t5609: F, t9794: F, t9793: F, t2619: F, t5635: F, t2689: F, t5618: F, t808: F, t9845: F, t1885: F, t9909: F, t2713: F, t3964: F, t5617: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13846 = t550 * t136;
    let t13848 = t124 * t1882;
    let t13857 = t9794 * t5609;
    let t13858 = t9793 * t13857;
    let t13887 = t5635 * t2619;
    let t13949 = t2689 * t5618;
    let t13955 = t808 * t5609;
    let t13956 = t9845 * t13955;
    let t13959 = t9909 * t1885;
    let t14013 = t3964 * t2713 * t5617;
    (t13846, t13848, t13857, t13858, t13887, t13949, t13956, t13959, t14013)
}
