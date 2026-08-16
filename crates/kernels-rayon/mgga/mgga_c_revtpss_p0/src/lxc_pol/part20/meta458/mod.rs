//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1746;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1747;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta458(t162: f64, t47038: f64, t47053: f64, t189: f64, t512: f64, t1340: f64, t40165: f64, t2626: f64, t9551: f64, t749: f64, t9363: f64, t268: f64, t520: f64, t39768: f64, t190: f64, t22: f64, t519: f64, t39762: f64, t1317: f64, t9545: f64, t40129: f64, t72: f64, t757: f64, t39807: f64, t39813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47055, t47057, t47059, t47061, t47064, t47065) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1746(t162, t47038, t47053, t189, t512, t1340, t40165, t2626, t9551, t749, t9363, t268, t520);
        let (t47067, t47070, t47072, t47074, t47076, t47078) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1747(t39768, t47065, t190, t22, t519, t39762, t1317, t9545, t1340, t40129, t72, t757, t9363);
        let (t47079, t47080) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1748(t47078, t39807, t39813, t47057, t47059, t47061, t47064, t47067, t47070, t47072, t47074, t47076);
    (t47055, t47057, t47059, t47061, t47064, t47067, t47070, t47072, t47074, t47076, t47079, t47080)
}
