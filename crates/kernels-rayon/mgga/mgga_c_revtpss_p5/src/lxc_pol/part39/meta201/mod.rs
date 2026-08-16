//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta201 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk826;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk827;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta201(t225: f64, t4376: f64, t4407: f64, t227: f64, t73: f64, t1544: f64, t853: f64, t775: f64, t4343: f64, t832: f64, t1553: f64, t1555: f64, t229: f64, t830: f64, t833: f64, t231: f64, t827: f64, t828: f64, t1559: f64, t221: f64, t2485: f64, t2484: f64, t2477: f64, t2672: f64, t2686: f64, t2704: f64, t2742: f64, t4345: f64, t4350: f64, t4355: f64, t4357: f64, t4359: f64, t4362: f64, t4368: f64, t4373: f64, t825: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4409, t4415, t4416, t4417, t4420, t4423) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk826(t225, t4376, t4407, t227, t73, t1544, t853, t775, t4343, t832, t1553, t1555, t229, t830, t833);
        let t4424 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk827(t231, t4423);
        let (t4426, t4430, t4433, t4435, t4439) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk828(t4424, t827, t828, t1559, t221, t2485, t2484, t1544, t775, t2477, t2672, t2686, t2704, t2742, t4345, t4350, t4355, t4357, t4359, t4362, t4368, t4373, t825, t851);
    (t4409, t4415, t4416, t4417, t4420, t4423, t4424, t4426, t4430, t4433, t4435, t4439)
}
