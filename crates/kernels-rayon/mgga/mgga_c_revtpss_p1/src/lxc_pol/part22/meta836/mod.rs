//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta836 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2962;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta836(t40690: f64, t5610: f64, t5618: f64, t9784: f64, t820: f64, t844: f64, t9991: f64, t13807: f64, t13767: f64, t2661: f64, t3829: f64, t48347: f64, t13776: f64, t9775: f64, t46644: f64, t5622: f64, t5614: f64, t9779: f64, t40488: f64, t13995: f64, t9962: f64, t2659: f64, t4086: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48829, t48833, t48836, t48837, t48845) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2962(t40690, t5610, t5618, t9784, t820, t844, t9991, t13807, t13767, t2661, t3829, t48347);
        let (t48847, t48849, t48851, t48853, t48855, t48862) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2963(t13776, t9775, t46644, t5622, t5614, t9779, t40488, t5610, t13995, t9962, t2659, t4086, t816);
    (t48829, t48833, t48836, t48837, t48845, t48847, t48849, t48851, t48853, t48855, t48862)
}
