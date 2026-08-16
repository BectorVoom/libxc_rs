//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2429;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2430;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2431;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta575(t2741: f64, t6019: f64, t5966: f64, t775: f64, t10698: f64, t828: f64, t1544: f64, t4343: f64, t2477: f64, t5984: f64, t800: f64, t5988: f64, t1548: f64, t10811: f64, t6037: f64, t18444: f64, t4364: f64, t4366: f64, t10846: f64, t10885: f64, t10888: f64, t10891: f64, t10900: f64, t2730: f64, t4362: f64, t851: f64, t10871: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18491, t18493, t18495, t18498, t18500, t18507, t18511) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2429(t2741, t6019, t5966, t775, t10698, t828, t1544, t4343, t2477, t5984, t800, t5988);
        let (t18515, t18521, t18524) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2430(t1548, t4343, t800, t10811, t6037, t18444, t4364, t4366, t10846, t10885, t10888, t10891, t10900, t18491, t18495, t18500, t18507, t18511, t2730, t4362, t851);
        let t18525 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2431(t10871, t836);
    (t18493, t18495, t18498, t18500, t18507, t18511, t18515, t18521, t18524, t18525)
}
