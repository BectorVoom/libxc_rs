//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta195 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk955;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk956;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk957;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta195<F: Float>(t1514: F, t625: F, t1513: F, t2339: F, t665: F, t1504: F, t2349: F, t658: F, t100: F, t2: F, t580: F, t1509: F, t2357: F, t661: F, t108: F, t105: F, t1505: F, t1507: F, t656: F, t662: F, t97: F, t114: F, t655: F, t2335: F, t2336: F, t69: F, t508: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4261, t4263, t4264, t4269, t4270, t4273, t4274, t4279) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk955::<F>(t1514, t625, t1513, t2339, t665, t1504, t2349, t658, t100, t2, t580, t1509, t2357);
        let (t4283, t4287) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk956::<F>(t4279, t661, t108, t2, t580, t105, t1505, t1507, t4270, t4274, t656, t662, t97);
        let (t4288, t4292) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk957::<F>(t114, t4287, t655, t2335, t2336, t4261, t4264, t69);
        let t4293 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk958::<F>(t4292, t508);
    (t4263, t4264, t4269, t4270, t4273, t4274, t4279, t4283, t4287, t4288, t4292, t4293)
}
