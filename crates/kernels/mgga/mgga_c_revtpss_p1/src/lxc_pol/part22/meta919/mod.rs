//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta919 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3129;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta919<F: Float>(t12077: F, t1647: F, t1086: F, t4930: F, t994: F, t342: F, t378: F, t43471: F, t3154: F, t43350: F, t16565: F, t989: F, t1071: F, t12046: F, t3298: F, t4743: F, t3316: F, t19602: F, t19607: F, t12166: F, t4746: F, t4980: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t55899, t55934, t55938, t55939, t55944) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3129::<F>(t12077, t1647, t1086, t4930, t994, t342, t378, t43471, t3154, t43350, t16565, t989);
        let (t55948, t55958, t55985, t55988, t55991, t56017, t56049) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3130::<F>(t1071, t12046, t342, t3298, t4743, t3316, t19602, t994, t19607, t12166, t1647, t4746, t4980);
    (t55899, t55934, t55938, t55939, t55944, t55948, t55958, t55985, t55988, t55991, t56017, t56049)
}
