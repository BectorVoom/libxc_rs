//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2455;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2456;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta661(t3115: f64, t3119: f64, t42793: f64, t11688: f64, t11922: f64, t4892: f64, t11249: f64, t3151: f64, t11722: f64, t3188: f64, t3046: f64, t3316: f64, t4891: f64, t11923: f64, t11933: f64, t11238: f64, t196: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t42795, t42798, t42804, t42816, t42830) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2455(t3115, t3119, t42793, t11688, t11922, t4892, t11249, t3151, t11722, t3188, t3046, t3316, t4891);
        let (t42833, t42859) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2456(t11923, t11933, t11238, t196);
    (t42795, t42798, t42804, t42816, t42830, t42833, t42859)
}
