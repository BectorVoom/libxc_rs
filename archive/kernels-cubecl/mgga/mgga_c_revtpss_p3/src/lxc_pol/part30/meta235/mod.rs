//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta235<F: Float>(t1247: F, t5265: F, t1263: F, t3367: F, t4181: F, t1042: F, t1032: F, t1770: F, t1246: F, t1774: F, t1122: F, t5062: F, t5065: F, t5067: F, t5070: F, t5107: F, t5111: F, t5189: F, t5191: F, t5194: F, t5196: F, t5200: F, t5204: F, t5209: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5266, t5268, t5269, t5270, t5273, t5274, t5277, t5278, t5279, t5284) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1069::<F>(t1247, t5265, t1263, t3367, t4181, t1042, t1032, t1770, t1246, t1774, t1122, t5062, t5065, t5067, t5070, t5107, t5111, t5189, t5191, t5194, t5196, t5200, t5204, t5209);
    (t5266, t5268, t5269, t5270, t5273, t5274, t5277, t5278, t5279, t5284)
}
