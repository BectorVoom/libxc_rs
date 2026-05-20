//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1178;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1179;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1180;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1181;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta182<F: Float>(t4311: F, t707: F, t2498: F, t2518: F, t2522: F, t2526: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t4300: F, t4301: F, t4304: F, t4307: F, t4310: F, t1522: F, t2398: F, t1568: F, t212: F, t45: F, t780: F, t689: F, t1569: F, t786: F, t789: F, t1469: F, t80: F, t4186: F, t606: F, t766: F, t83: F, zeta_threshold: F, t57: F, t770: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4313, t4314) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1178::<F>(t4311, t707, t2498, t2518, t2522, t2526, t2562, t2569, t2579, t2587, t2610, t4300, t4301, t4304, t4307, t4310);
        let (t4316, t4321) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1179::<F>(t1522, t2398, t1568, t212);
        let (t4322, t4323, t4325, t4326, t4328, t4334, t4335) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1180::<F>(t45, t4321, t780, t689, t1569, t786, t789, t1469, t80, t4186, t606, t766, t83, zeta_threshold);
        let t4343 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1181::<F>(t57, t4186, t4335, t606, t770, t4334, zeta_threshold);
    (t4313, t4314, t4316, t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4343)
}
