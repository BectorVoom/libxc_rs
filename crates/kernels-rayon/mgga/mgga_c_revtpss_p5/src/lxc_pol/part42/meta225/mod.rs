//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk870;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk871;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk872;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk873;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta225(t225: f64, t6005: f64, t2638: f64, t5966: f64, t5962: f64, t832: f64, t1553: f64, t1555: f64, t227: f64, t229: f64, t231: f64, t827: f64, t828: f64, t2723: f64, t5977: f64, t855: f64, t1544: f64, t4365: f64, t2747: f64, t2702: f64, t2716: f64, t2721: f64, t2739: f64, t2745: f64, t4350: f64, t4355: f64, t4357: f64, t4431: f64, t825: f64, t851: f64, t2672: f64, t2686: f64, t2691: f64, t2730: f64, t4359: f64, t4373: f64, t4455: f64, t5980: f64, t5985: f64, t5989: f64, t5993: f64, t799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6006, t6010, t6013, t6016, t6017) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk870(t225, t6005, t2638, t5966, t5962, t832, t1553, t1555, t227, t229, t231);
        let (t6019, t6022) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk871(t6017, t827, t828, t2723, t5977);
        let (t6024, t6030, t6035) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk872(t6022, t827, t828, t5962, t855, t1544, t231);
        let (t6037, t6040) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk873(t4365, t6035, t2747, t2702, t2716, t2721, t2739, t2745, t4350, t4355, t4357, t4431, t6019, t6024, t6030, t825, t851);
        let t6041 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk874(t2672, t2686, t2691, t2730, t4359, t4373, t4455, t5980, t5985, t5989, t5993, t6040, t799, t825, t851);
    (t6006, t6010, t6013, t6016, t6017, t6019, t6022, t6024, t6030, t6035, t6037, t6041)
}
