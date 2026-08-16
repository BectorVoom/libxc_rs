//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta189 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1147;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1148;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1149;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta189(t4424: f64, t827: f64, t828: f64, t1559: f64, t221: f64, t2485: f64, t2484: f64, t1544: f64, t775: f64, t2477: f64, t2672: f64, t2686: f64, t2704: f64, t2742: f64, t4345: f64, t4350: f64, t4355: f64, t4357: f64, t4359: f64, t4362: f64, t4368: f64, t4373: f64, t825: f64, t851: f64, t1548: f64, t800: f64, t4365: f64, t837: f64, t4364: f64, t125: f64, t2747: f64, t1549: f64, t2703: f64, t124: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4426, t4430, t4431, t4433) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1147(t4424, t827, t828, t1559, t221, t2485, t2484, t1544, t775);
        let (t4435, t4439) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1148(t2477, t4433, t828, t2672, t2686, t2704, t2742, t4345, t4350, t4355, t4357, t4359, t4362, t4368, t4373, t4426, t4431, t825, t851);
        let (t4442, t4447, t4450) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1149(t1548, t775, t800, t4365, t837, t4364, t125, t1544);
        let (t4452, t4455, t4457) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1150(t4450, t837, t2747, t1549, t2703, t124, t4343);
    (t4426, t4430, t4433, t4435, t4439, t4442, t4447, t4450, t4452, t4455, t4457)
}
