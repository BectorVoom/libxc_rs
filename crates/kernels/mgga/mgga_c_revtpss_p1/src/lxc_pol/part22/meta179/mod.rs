//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1171;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta179<F: Float>(t1310: F, t1518: F, t1514: F, t625: F, t1513: F, t2339: F, t665: F, t1504: F, t2349: F, t658: F, t100: F, t2: F, t580: F, t1509: F, t2357: F) -> (F, F, F, F, F, F, F, F, F) {
        let t4257 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1171::<F>(t1310, t1518);
        let (t4261, t4263, t4264, t4269, t4270, t4273, t4274, t4279) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1172::<F>(t1514, t625, t1513, t2339, t665, t1504, t2349, t658, t100, t2, t580, t1509, t2357);
    (t4257, t4261, t4263, t4264, t4269, t4270, t4273, t4274, t4279)
}
