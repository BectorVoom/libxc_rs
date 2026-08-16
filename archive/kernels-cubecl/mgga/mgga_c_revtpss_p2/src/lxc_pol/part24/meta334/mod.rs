//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1165;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta334<F: Float>(t1544: F, t5962: F, t2477: F, t828: F, t23177: F, t827: F, t23245: F, t18426: F, t2747: F, t6035: F, t4364: F, t4365: F, t6017: F, t14586: F, t18444: F, t10756: F, t10758: F, t14780: F, t14817: F, t14820: F, t14839: F, t18350: F, t18354: F, t2745: F, t4362: F, t825: F, t851: F) -> (F, F, F, F, F, F, F, F) {
        let (t23279, t23281, t23285, t23289, t23293, t23297) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1165::<F>(t1544, t5962, t2477, t828, t23177, t827, t23245, t18426, t2747, t6035, t4364, t4365, t6017);
        let (t23301, t23310) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1166::<F>(t14586, t18444, t4364, t10756, t10758, t14780, t14817, t14820, t14839, t18350, t18354, t23281, t23285, t23289, t23293, t23297, t2745, t4362, t825, t851);
    (t23279, t23281, t23285, t23289, t23293, t23297, t23301, t23310)
}
