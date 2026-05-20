//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta148 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk931;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk932;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta148<F: Float>(t1310: F, t1518: F, t1514: F, t625: F, t1513: F, t2339: F, t665: F, t1504: F, t2349: F, t658: F, t100: F, t2: F, t580: F, t1509: F, t2357: F, t661: F, t108: F, t105: F, t1505: F, t1507: F, t656: F, t662: F, t97: F, t114: F, t655: F, t2335: F, t2336: F, t69: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4257, t4261, t4263, t4264, t4269, t4270, t4273) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk931::<F>(t1310, t1518, t1514, t625, t1513, t2339, t665, t1504, t2349, t658, t100, t2);
        let (t4279, t4280, t4284, t4287) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk932::<F>(t4273, t580, t1509, t2357, t661, t108, t2, t105, t1505, t1507, t4270, t656, t662, t97);
        let (t4288, t4292) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk933::<F>(t114, t4287, t655, t2335, t2336, t4261, t4264, t69);
    (t4257, t4261, t4263, t4264, t4269, t4279, t4280, t4284, t4287, t4288, t4292)
}
