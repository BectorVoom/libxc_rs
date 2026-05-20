//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta764 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2560;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2561;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta764<F: Float>(t1086: F, t4930: F, t994: F, t342: F, t378: F, t43471: F, t3154: F, t43350: F, t3298: F, t4743: F, t3316: F, t19602: F, t19607: F, t12166: F, t1647: F, t4746: F, t4980: F, t379: F, t2435: F, t5048: F, t5053: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t55934, t55938, t55939, t55958, t55985, t55988) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2560::<F>(t1086, t4930, t994, t342, t378, t43471, t3154, t43350, t3298, t4743, t3316, t19602);
        let (t55991, t56017, t56049, t56087, t56176) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2561::<F>(t19607, t994, t12166, t1647, t4746, t4980, t342, t379, t2435, t5048);
        let t56183 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2562::<F>(t2435, t5053);
    (t55934, t55938, t55939, t55958, t55985, t55988, t55991, t56017, t56049, t56087, t56176, t56183)
}
