//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta868 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3025;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta868(t14728: f64, t9775: f64, t1549: f64, t40861: f64, t14779: f64, t40721: f64, t221: f64, t40724: f64, t10777: f64, t14787: f64, t14495: f64, t40834: f64, t826: f64, t241: f64, t820: f64, t849: f64, t14900: f64, t14923: f64, t10811: f64, t14914: f64, t14788: f64, t10886: f64, t14652: f64, t808: f64, t14746: f64, t2703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50939, t50941, t50943, t50945, t50947, t50954) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3025(t14728, t9775, t1549, t40861, t14779, t40721, t221, t40724, t10777, t14787, t14495, t40834, t826);
        let (t50957, t50966, t50968, t50974, t50977, t50982) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3026(t241, t820, t849, t14900, t14923, t10811, t14914, t14788, t10886, t14652, t808, t14746, t2703);
    (t50939, t50941, t50943, t50945, t50947, t50954, t50957, t50966, t50968, t50974, t50977, t50982)
}
