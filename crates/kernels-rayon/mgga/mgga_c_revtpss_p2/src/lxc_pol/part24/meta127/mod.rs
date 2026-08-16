//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta127 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk671;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk672;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk673;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta127(t1592: f64, t3109: f64, t247: f64, t1063: f64, t1670: f64, t3172: f64, t1041: f64, t1065: f64, t1651: f64, t1062: f64, t1659: f64, t3204: f64, t127: f64, t1663: f64, t371: f64, t1025: f64, t225: f64, t4746: f64, t366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4817, t4818, t4820, t4821, t4823, t4834) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk671(t1592, t3109, t247, t1063, t1670, t3172, t1041, t1065, t1651, t1062, t1659);
        let t4837 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk672(t1062, t3204);
        let (t4845, t4846, t4857) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk673(t127, t1663, t371, t1025, t225, t4746);
        let t4858 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk674(t366, t4857);
    (t4817, t4818, t4820, t4821, t4823, t4834, t4837, t4845, t4846, t4857, t4858)
}
