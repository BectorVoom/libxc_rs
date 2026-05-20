//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2499;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta684<F: Float>(t371: F, t481: F, t482: F, t9291: F, t12627: F, t1284: F, t3624: F, t12910: F, t12911: F, t12916: F, t12640: F, t127: F, t12866: F, t3630: F, t3712: F, t12809: F, t12811: F, t12952: F, t3172: F, t3711: F, t12901: F, t13033: F, t13042: F, t13047: F, t3555: F, t3781: F, t5330: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44607, t44609, t44616, t44624, t44634) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2499::<F>(t371, t481, t482, t9291, t12627, t1284, t3624, t12910, t12911, t12916, t12640, t127, t12866, t3630, t3712);
        let (t44637, t44649, t44658, t44661, t44664) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2500::<F>(t12809, t12811, t12916, t12952, t3172, t3711, t12901, t13033, t13042, t13047, t3555, t3781, t5330);
    (t44607, t44609, t44616, t44624, t44634, t44637, t44649, t44658, t44661, t44664)
}
