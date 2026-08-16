//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta919 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3129;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta919(t12077: f64, t1647: f64, t1086: f64, t4930: f64, t994: f64, t342: f64, t378: f64, t43471: f64, t3154: f64, t43350: f64, t16565: f64, t989: f64, t1071: f64, t12046: f64, t3298: f64, t4743: f64, t3316: f64, t19602: f64, t19607: f64, t12166: f64, t4746: f64, t4980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t55899, t55934, t55938, t55939, t55944) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3129(t12077, t1647, t1086, t4930, t994, t342, t378, t43471, t3154, t43350, t16565, t989);
        let (t55948, t55958, t55985, t55988, t55991, t56017, t56049) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3130(t1071, t12046, t342, t3298, t4743, t3316, t19602, t994, t19607, t12166, t1647, t4746, t4980);
    (t55899, t55934, t55938, t55939, t55944, t55948, t55958, t55985, t55988, t55991, t56017, t56049)
}
