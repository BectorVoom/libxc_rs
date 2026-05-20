//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1567;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1568;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta307<F: Float>(t2664: F, t808: F, t10744: F, t2693: F, t2710: F, t2713: F, t2706: F, t775: F, t800: F, t810: F, t9784: F, t9789: F, t235: F, t2783: F, t2453: F) -> (F, F, F, F, F, F, F, F) {
        let (t10745, t10746, t10749, t10752, t10756, t10758) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1567::<F>(t2664, t808, t10744, t2693, t2710, t2713, t2706, t775, t800, t810, t9784, t9789);
        let (t10759, t10760) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1568::<F>(t235, t2783, t2453);
    (t10745, t10746, t10749, t10752, t10756, t10758, t10759, t10760)
}
