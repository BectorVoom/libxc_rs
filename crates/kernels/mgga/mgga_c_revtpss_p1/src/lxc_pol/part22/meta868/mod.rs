//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta868 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3025;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta868<F: Float>(t14728: F, t9775: F, t1549: F, t40861: F, t14779: F, t40721: F, t221: F, t40724: F, t10777: F, t14787: F, t14495: F, t40834: F, t826: F, t241: F, t820: F, t849: F, t14900: F, t14923: F, t10811: F, t14914: F, t14788: F, t10886: F, t14652: F, t808: F, t14746: F, t2703: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50939, t50941, t50943, t50945, t50947, t50954) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3025::<F>(t14728, t9775, t1549, t40861, t14779, t40721, t221, t40724, t10777, t14787, t14495, t40834, t826);
        let (t50957, t50966, t50968, t50974, t50977, t50982) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3026::<F>(t241, t820, t849, t14900, t14923, t10811, t14914, t14788, t10886, t14652, t808, t14746, t2703);
    (t50939, t50941, t50943, t50945, t50947, t50954, t50957, t50966, t50968, t50974, t50977, t50982)
}
