//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1959;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1960;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta442<F: Float>(t14122: F, t4086: F, t543: F, t2782: F, t555: F, t5658: F, t1882: F, t4114: F, t2482: F, t122: F, t4003: F, t72: F, t1398: F, t676: F, t10069: F, t5737: F, t10015: F, t10020: F, t10027: F, t10032: F, t10035: F, t10041: F, t10044: F, t14116: F, t14120: F, t4004: F, t5735: F, t5745: F, t9840: F) -> (F, F, F, F, F, F, F, F) {
        let (t14124, t14126, t14127) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1959::<F>(t14122, t4086, t543, t2782, t555, t5658);
        let (t14129, t14131, t14140, t14141, t14143, t14144) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1960::<F>(t14127, t4086, t543, t2782, t1882, t4114, t2482, t122, t4003, t72, t1398, t676);
        let (t14145, t14151) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1961::<F>(t14143, t14144, t14141, t10069, t5737, t10015, t10020, t10027, t10032, t10035, t10041, t10044, t14116, t14120, t14126, t14131, t4004, t5735, t5745, t9840);
    (t14124, t14127, t14129, t14140, t14141, t14143, t14145, t14151)
}
