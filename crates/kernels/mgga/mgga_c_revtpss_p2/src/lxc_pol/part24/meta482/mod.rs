//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1472;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta482<F: Float>(t17361: F, t5293: F, t1261: F, t12879: F, t247: F, t6425: F, t17416: F, t5391: F, t44693: F, t6421: F, t1222: F, t6652: F, t697: F, t1235: F, t371: F, t6645: F, t676: F, t17307: F, t1803: F, t11262: F, t3711: F, t6618: F, t3609: F, t69692: F, t5381: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t69971, t70032, t70112, t70133, t70225) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1472::<F>(t17361, t5293, t1261, t12879, t247, t6425, t17416, t5391, t44693, t6421, t1222, t6652, t697);
        let (t70263, t70267, t70278, t70319, t70405) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1473::<F>(t1235, t371, t6645, t676, t17307, t1803, t11262, t3711, t6618, t3609, t69692, t17416, t5381);
    (t69971, t70032, t70112, t70133, t70225, t70263, t70267, t70278, t70319, t70405)
}
