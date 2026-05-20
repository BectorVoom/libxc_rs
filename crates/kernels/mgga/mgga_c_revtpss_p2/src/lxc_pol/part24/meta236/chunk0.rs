//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 996/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk996<F: Float>(t13846: F, t220: F, t124: F, t1882: F, t5609: F, t9794: F, t9793: F, t2619: F, t5635: F, t2689: F, t5618: F, t808: F) -> (F, F, F, F, F, F) {
    let t13847 = t13846 * t220;
    let t13848 = t124 * t1882;
    let t13857 = t9794 * t5609;
    let t13858 = t9793 * t13857;
    let t13887 = t5635 * t2619;
    let t13949 = t2689 * t5618;
    let t13955 = t808 * t5609;
    (t13847, t13848, t13858, t13887, t13949, t13955)
}
