//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta914 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3121;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta914(t11792: f64, t4845: f64, t15749: f64, t3224: f64, t11922: f64, t16039: f64, t3115: f64, t11859: f64, t15610: f64, t1032: f64, t1040: f64, t15886: f64, t15690: f64, t3153: f64, t372: f64, t11921: f64, t15716: f64, t15717: f64, t247: f64, t1041: f64, t1670: f64, t42994: f64, t15786: f64, t4892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55152, t55154, t55171, t55182, t55195) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3121(t11792, t4845, t15749, t3224, t11922, t16039, t3115, t11859, t15610, t1032, t1040, t15886);
        let (t55209, t55233, t55247, t55265) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3122(t15690, t3153, t372, t11921, t15716, t15717, t247, t1041, t1670, t42994, t11922, t15786, t4892);
    (t55152, t55154, t55171, t55182, t55195, t55209, t55233, t55247, t55265)
}
