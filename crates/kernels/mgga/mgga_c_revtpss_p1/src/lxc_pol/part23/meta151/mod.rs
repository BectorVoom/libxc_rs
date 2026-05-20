//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk938;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk939;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk940;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta151<F: Float>(t45: F, t4321: F, t780: F, t689: F, t1569: F, t786: F, t789: F, t1469: F, t80: F, t4186: F, t606: F, t766: F, t83: F, zeta_threshold: F, t57: F, t770: F, t828: F, t855: F, t1544: F, t221: F, t2675: F, t2674: F, t1558: F, t243: F, t231: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4322, t4323, t4325, t4326, t4328, t4334, t4335) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk938::<F>(t45, t4321, t780, t689, t1569, t786, t789, t1469, t80, t4186, t606, t766, t83, zeta_threshold);
        let t4343 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk939::<F>(t57, t4186, t4335, t606, t770, t4334, zeta_threshold);
        let (t4345, t4349, t4350, t4352) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk940::<F>(t4343, t828, t855, t1544, t221, t2675, t2674, t1558, t243);
        let t4353 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk941::<F>(t231, t4352);
    (t4322, t4323, t4325, t4326, t4328, t4335, t4343, t4345, t4349, t4350, t4352, t4353)
}
