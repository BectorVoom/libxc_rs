//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk996;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta236(t13846: f64, t220: f64, t124: f64, t1882: f64, t5609: f64, t9794: f64, t9793: f64, t2619: f64, t5635: f64, t2689: f64, t5618: f64, t808: f64, t9845: f64, t1885: f64, t9909: f64, t4000: f64, t820: f64, t844: f64, t2713: f64, t3964: f64, t5617: f64, t5665: f64, t9976: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13847, t13848, t13858, t13887, t13949, t13955) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk996(t13846, t220, t124, t1882, t5609, t9794, t9793, t2619, t5635, t2689, t5618, t808);
        let (t13956, t13959, t13999, t14013, t14043) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk997(t13955, t9845, t1885, t9909, t4000, t820, t844, t2713, t3964, t5617, t5665, t9976);
    (t13847, t13848, t13858, t13887, t13949, t13955, t13956, t13959, t13999, t14013, t14043)
}
