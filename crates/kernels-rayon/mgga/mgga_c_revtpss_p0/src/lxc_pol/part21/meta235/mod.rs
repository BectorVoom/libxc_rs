//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1386;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1387;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1388;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta235(t2782: f64, t5737: f64, t1883: f64, t72: f64, t686: f64, t4101: f64, t225: f64, t3999: f64, t213: f64, t4086: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5738, t5740, t5741, t5742, t5744) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1386(t2782, t5737, t1883, t72, t686, t4101, t225, t3999);
        let t5745 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1387(t213, t5744);
        let t5755 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1388(t213, t4086);
    (t5738, t5740, t5741, t5742, t5744, t5745, t5755)
}
