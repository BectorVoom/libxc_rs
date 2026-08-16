//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1472;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta482(t17361: f64, t5293: f64, t1261: f64, t12879: f64, t247: f64, t6425: f64, t17416: f64, t5391: f64, t44693: f64, t6421: f64, t1222: f64, t6652: f64, t697: f64, t1235: f64, t371: f64, t6645: f64, t676: f64, t17307: f64, t1803: f64, t11262: f64, t3711: f64, t6618: f64, t3609: f64, t69692: f64, t5381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69971, t70032, t70112, t70133, t70225) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1472(t17361, t5293, t1261, t12879, t247, t6425, t17416, t5391, t44693, t6421, t1222, t6652, t697);
        let (t70263, t70267, t70278, t70319, t70405) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1473(t1235, t371, t6645, t676, t17307, t1803, t11262, t3711, t6618, t3609, t69692, t17416, t5381);
    (t69971, t70032, t70112, t70133, t70225, t70263, t70267, t70278, t70319, t70405)
}
