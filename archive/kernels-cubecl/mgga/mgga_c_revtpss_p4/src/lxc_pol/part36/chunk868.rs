//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 868/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk868<F: Float>(t2689: F, t5618: F, t5609: F, t808: F, t9845: F, t1885: F, t9909: F, t2713: F, t3964: F, t5617: F, t5665: F, t9976: F) -> (F, F, F, F, F) {
    let t13949 = t2689 * t5618;
    let t13955 = t808 * t5609;
    let t13956 = t9845 * t13955;
    let t13959 = t9909 * t1885;
    let t14013 = t3964 * t2713 * t5617;
    let t14043 = t9976 * t5665;
    (t13949, t13956, t13959, t14013, t14043)
}
