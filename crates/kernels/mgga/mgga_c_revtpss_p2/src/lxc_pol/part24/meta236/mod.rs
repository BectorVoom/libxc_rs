//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk996;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta236<F: Float>(t13846: F, t220: F, t124: F, t1882: F, t5609: F, t9794: F, t9793: F, t2619: F, t5635: F, t2689: F, t5618: F, t808: F, t9845: F, t1885: F, t9909: F, t4000: F, t820: F, t844: F, t2713: F, t3964: F, t5617: F, t5665: F, t9976: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13847, t13848, t13858, t13887, t13949, t13955) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk996::<F>(t13846, t220, t124, t1882, t5609, t9794, t9793, t2619, t5635, t2689, t5618, t808);
        let (t13956, t13959, t13999, t14013, t14043) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk997::<F>(t13955, t9845, t1885, t9909, t4000, t820, t844, t2713, t3964, t5617, t5665, t9976);
    (t13847, t13848, t13858, t13887, t13949, t13955, t13956, t13959, t13999, t14013, t14043)
}
