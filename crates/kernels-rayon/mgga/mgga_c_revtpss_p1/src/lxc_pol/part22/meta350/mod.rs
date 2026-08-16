//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1841;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1842;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1843;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta350(t1025: f64, t11817: f64, t271: f64, t2857: f64, t283: f64, t66: f64, t3298: f64, t994: f64, t4891: f64, t3154: f64, t999: f64, t1086: f64, t3046: f64, t3090: f64, t3316: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11818, t11821, t11852) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1841(t1025, t11817, t271, t2857, t283);
        let (t11853, t11858, t11859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1842(t11852, t66, t3298, t994, t4891);
        let (t11860, t11865, t11866) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1843(t3154, t999, t1086, t3046, t3090);
        let (t11874, t11875) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1844(t3316, t994, t4891);
    (t11818, t11821, t11852, t11853, t11858, t11859, t11860, t11865, t11866, t11874, t11875)
}
