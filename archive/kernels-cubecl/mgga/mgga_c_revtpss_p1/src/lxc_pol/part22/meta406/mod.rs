//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2001;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta406<F: Float>(t14114: F, t4104: F, t10073: F, t5737: F, t1419: F, t1882: F, t4086: F, t543: F, t2782: F, t555: F, t5658: F, t4114: F, t2482: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14116, t14120, t14122, t14124, t14126, t14127, t14129, t14131, t14140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2001::<F>(t14114, t4104, t10073, t5737, t1419, t1882, t4086, t543, t2782, t555, t5658, t4114);
        let t14141 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2002::<F>(t14140, t2482);
    (t14116, t14120, t14122, t14124, t14126, t14127, t14129, t14131, t14140, t14141)
}
