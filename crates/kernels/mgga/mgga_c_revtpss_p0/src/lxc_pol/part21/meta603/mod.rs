//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2330;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2331;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2332;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta603<F: Float>(t2710: F, t2793: F, t39494: F, t2804: F, t874: F, t9288: F, t10535: F, t231: F, t2645: F, t281: F, t68: F, t211: F, t9644: F, t209: F, t234: F, t251: F, t268: F, t8779: F, t39497: F, t875: F, t10530: F, t2723: F, t39583: F, t2798: F, t39599: F, t624: F, t836: F, t2722: F, t10529: F, t2453: F, t10523: F, t10542: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39633, t39635, t39640, t39643) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2330::<F>(t2710, t2793, t39494, t2804, t874, t9288, t10535, t231, t2645, t281, t68, t211, t9644);
        let t39644 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2331::<F>(t209, t39643);
        let (t39649, t39652, t39662, t39668) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2332::<F>(t234, t251, t268, t39644, t8779, t39497, t874, t875, t10530, t2723, t39583, t231, t2798, t39599);
        let (t39673, t39678, t39683, t39685) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2333::<F>(t10535, t231, t281, t624, t836, t2722, t68, t10529, t2453, t2723, t10523, t10542);
    (t39633, t39635, t39640, t39644, t39649, t39652, t39662, t39668, t39673, t39678, t39683, t39685)
}
