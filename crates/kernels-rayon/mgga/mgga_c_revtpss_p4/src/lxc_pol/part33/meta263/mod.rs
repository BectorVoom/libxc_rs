//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1177;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta263(t1444: f64, t2022: f64, t7296: f64, t1385: f64, t1426: f64, t1398: f64, t543: f64, t545: f64, t7274: f64, t2028: f64, t1445: f64, t2027: f64, t2030: f64, t213: f64, t561: f64, t7245: f64, t7248: f64, t7275: f64, t7279: f64, t7288: f64, t7291: f64, t7292: f64, t7295: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t7298, t7301) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1177(t1444, t2022, t7296, t1385, t1426);
        let (t7303, t7304, t7307, t7308, t7311) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1178(t1398, t2022, t543, t7301, t545, t7274, t2028, t1445, t2027, t2030, t213, t561, t7245, t7248, t7275, t7279, t7288, t7291, t7292, t7295, t7298);
    (t7298, t7301, t7303, t7304, t7307, t7308, t7311)
}
