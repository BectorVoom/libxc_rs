//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta844 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2978;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta844(t10142: f64, t14113: f64, t49180: f64, t10136: f64, t14239: f64, t10119: f64, t4101: f64, t5740: f64, t9288: f64, t1419: f64, t5658: f64, t2782: f64, t4086: f64, t543: f64, t40270: f64, t5737: f64, t13920: f64, t555: f64, t10073: f64, t14207: f64, t47973: f64, t10090: f64, t13805: f64, t1882: f64, t2482: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49189, t49198, t49200, t49203, t49208) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2978(t10142, t14113, t49180, t10136, t14239, t10119, t4101, t5740, t9288, t1419, t5658, t2782, t4086, t543);
        let (t49210, t49213, t49238, t49242, t49248) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2979(t40270, t5737, t13920, t555, t10073, t14207, t2782, t4086, t47973, t543, t10090, t13805, t1882, t2482, t686, t72);
    (t49189, t49198, t49200, t49203, t49208, t49210, t49213, t49238, t49242, t49248)
}
