//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta688 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2507;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta688<F: Float>(t12984: F, t3667: F, t12976: F, t3678: F, t12963: F, t1235: F, t127: F, t12970: F, t371: F, t126: F, t13099: F, t12257: F, t1261: F, t247: F, t12879: F, t3372: F, t3368: F, t1222: F, t12287: F, t17240: F, t12881: F, t3647: F, t1224: F, t12268: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44884, t44886, t44888, t44892, t44898) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2507::<F>(t12984, t3667, t12976, t3678, t12963, t1235, t127, t12970, t371, t126, t13099, t12257, t1261, t247);
        let (t44902, t44906, t44912, t44917, t44919) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2508::<F>(t1261, t12879, t247, t3372, t3368, t1222, t12287, t17240, t12881, t3647, t1224, t12268);
    (t44884, t44886, t44888, t44892, t44898, t44902, t44906, t44912, t44917, t44919)
}
