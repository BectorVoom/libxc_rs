//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta863 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3015;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta863<F: Float>(t10811: F, t14707: F, t14874: F, t14673: F, t40731: F, t40593: F, t4447: F, t4462: F, t10760: F, t40763: F, t4353: F, t1559: F, t775: F, t40834: F, t854: F, t14587: F, t2735: F, t40798: F, t826: F, t10777: F, t10779: F, t2749: F, t50412: F, t14686: F, t837: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t50600, t50602, t50604, t50606, t50608, t50611, t50613) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3015::<F>(t10811, t14707, t14874, t14673, t40731, t40593, t4447, t4462, t10760, t40763, t4353, t1559, t775);
        let (t50615, t50619, t50628, t50632) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3016::<F>(t40834, t50613, t854, t14587, t2735, t40798, t826, t10777, t10779, t2749, t50412, t14686, t837);
    (t50600, t50602, t50604, t50606, t50608, t50611, t50613, t50615, t50619, t50628, t50632)
}
