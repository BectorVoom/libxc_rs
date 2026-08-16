//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1710;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta357(t3154: f64, t999: f64, t11659: f64, t3117: f64, t1086: f64, t3046: f64, t3090: f64, t1043: f64, t3075: f64, t1045: f64, t3316: f64, t994: f64, t4891: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11861, t11862, t11865, t11866) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1710(t3154, t999, t11659, t3117, t1086, t3046, t3090);
        let (t11869, t11870, t11871, t11874, t11875) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1711(t1043, t3075, t1045, t3117, t3316, t994, t4891);
    (t11861, t11862, t11865, t11866, t11869, t11870, t11871, t11874, t11875)
}
