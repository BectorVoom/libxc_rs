//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2115;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2116;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta452(t15135: f64, t2908: f64, t141: f64, t11341: f64, t15140: f64, t15145: f64, t930: f64, t15149: f64, t1593: f64, t2435: f64, t4584: f64, t689: f64, t13312: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15177, t15178, t15180, t15181, t15183, t15184, t15186, t15187, t15189) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2115(t15135, t2908, t141, t11341, t15140, t15145, t930, t15149, t1593, t2435);
        let t15191 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2116(t4584, t689);
        let (t15192, t15193) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2117(t15191, t13312, t905);
    (t15177, t15178, t15180, t15181, t15183, t15184, t15186, t15187, t15189, t15191, t15192, t15193)
}
