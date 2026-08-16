//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1959;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1960;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta442(t14122: f64, t4086: f64, t543: f64, t2782: f64, t555: f64, t5658: f64, t1882: f64, t4114: f64, t2482: f64, t122: f64, t4003: f64, t72: f64, t1398: f64, t676: f64, t10069: f64, t5737: f64, t10015: f64, t10020: f64, t10027: f64, t10032: f64, t10035: f64, t10041: f64, t10044: f64, t14116: f64, t14120: f64, t4004: f64, t5735: f64, t5745: f64, t9840: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14124, t14126, t14127) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1959(t14122, t4086, t543, t2782, t555, t5658);
        let (t14129, t14131, t14140, t14141, t14143, t14144) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1960(t14127, t4086, t543, t2782, t1882, t4114, t2482, t122, t4003, t72, t1398, t676);
        let (t14145, t14151) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1961(t14143, t14144, t14141, t10069, t5737, t10015, t10020, t10027, t10032, t10035, t10041, t10044, t14116, t14120, t14126, t14131, t4004, t5735, t5745, t9840);
    (t14124, t14127, t14129, t14140, t14141, t14143, t14145, t14151)
}
