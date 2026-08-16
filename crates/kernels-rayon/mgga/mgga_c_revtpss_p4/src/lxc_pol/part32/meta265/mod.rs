//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1118;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1119;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta265(t2102: f64, t72: f64, t686: f64, t7284: f64, t7289: f64, t1444: f64, t2097: f64, t7296: f64, t1398: f64, t543: f64, t7301: f64, t545: f64, t7506: f64, t2028: f64, t1445: f64, t2027: f64, t2103: f64, t213: f64, t561: f64, t7292: f64, t7295: f64, t7495: f64, t7498: f64, t7507: f64, t7511: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7514, t7515) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1118(t2102, t72, t686);
        let (t7517, t7519, t7523, t7527, t7528, t7531, t7532) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1119(t7284, t7515, t7289, t1444, t2097, t7296, t1398, t543, t7301, t545, t7506, t2028);
        let t7535 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1120(t1445, t2027, t2103, t213, t561, t7292, t7295, t7495, t7498, t7507, t7511, t7517, t7519, t7523, t7528, t7532);
    (t7514, t7515, t7517, t7519, t7523, t7527, t7528, t7531, t7532, t7535)
}
