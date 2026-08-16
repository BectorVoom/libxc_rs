//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta836 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2962;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta836<F: Float>(t40690: F, t5610: F, t5618: F, t9784: F, t820: F, t844: F, t9991: F, t13807: F, t13767: F, t2661: F, t3829: F, t48347: F, t13776: F, t9775: F, t46644: F, t5622: F, t5614: F, t9779: F, t40488: F, t13995: F, t9962: F, t2659: F, t4086: F, t816: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48829, t48833, t48836, t48837, t48845) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2962::<F>(t40690, t5610, t5618, t9784, t820, t844, t9991, t13807, t13767, t2661, t3829, t48347);
        let (t48847, t48849, t48851, t48853, t48855, t48862) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2963::<F>(t13776, t9775, t46644, t5622, t5614, t9779, t40488, t5610, t13995, t9962, t2659, t4086, t816);
    (t48829, t48833, t48836, t48837, t48845, t48847, t48849, t48851, t48853, t48855, t48862)
}
