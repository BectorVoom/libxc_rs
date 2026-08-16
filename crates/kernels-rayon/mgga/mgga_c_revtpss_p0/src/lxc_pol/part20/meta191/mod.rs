//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta191 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk949;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk950;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta191(t543: f64, t9898: f64, t1390: f64, t828: f64, t221: f64, t4019: f64, t4057: f64, t4018: f64, t1386: f64, t2681: f64, t820: f64, t1401: f64, t4003: f64, t4000: f64, t843: f64, t4006: f64, t136: f64, t4011: f64, t3829: f64, t3978: f64, t3970: f64, t3989: f64, t1388: f64, t3934: f64, t4002: f64, t5671: f64, t9828: f64, t9832: f64, t9837: f64, t9842: f64, t9847: f64, t9893: f64, t9896: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9899, t9901, t9905, t9906, t9909, t9910) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk949(t543, t9898, t1390, t828, t221, t4019, t4057, t4018, t1386, t2681, t820, t1401);
        let (t9912, t9914, t9918, t9919, t9921, t9923) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk950(t4003, t9898, t1390, t828, t4000, t820, t843, t4006, t136, t4011, t221, t3829);
        let t9928 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk951(t3978, t9923, t3970, t3989, t1388, t3934, t4002, t5671, t9828, t9832, t9837, t9842, t9847, t9893, t9896, t9901, t9906, t9910, t9914, t9919);
    (t9899, t9901, t9905, t9909, t9912, t9914, t9918, t9921, t9923, t9928)
}
