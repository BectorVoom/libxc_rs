//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1645;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta334<F: Float>(t11300: F, t2926: F, t11299: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F, t923: F, t11156: F, t2908: F, t141: F, t11165: F, t930: F, t2912: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11301, t11303, t11304, t11315) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1645::<F>(t11300, t2926, t11299, t11132, t11134, t11136, t11138, t11140, t11147, t11153, t11158, t11162, t11167, t11171);
        let (t11316, t11318, t11319, t11321, t11322, t11326) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1646::<F>(t11315, t923, t11156, t2908, t141, t11165, t930, t2912, t698);
    (t11301, t11303, t11304, t11315, t11316, t11318, t11319, t11321, t11322, t11326)
}
