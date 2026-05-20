//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1302;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1303;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta358<F: Float>(t10638: F, t231: F, t268: F, t2798: F, t675: F, t2645: F, t837: F, t2782: F, t2797: F, t10115: F, t883: F, t2482: F, t2811: F, t39588: F, t686: F, t72: F, t2710: F, t2793: F, t39494: F, t2804: F, t874: F, t9288: F, t10535: F, t281: F, t68: F, t211: F, t9644: F, t209: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39617, t39620, t39622, t39624, t39629) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1302::<F>(t10638, t231, t268, t2798, t675, t2645, t837, t2782, t2797, t10115, t883, t2482, t2811, t39588, t686, t72);
        let (t39633, t39635, t39640, t39643) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1303::<F>(t2710, t2793, t39494, t2804, t874, t9288, t10535, t231, t2645, t281, t68, t211, t9644);
        let t39644 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1304::<F>(t209, t39643);
    (t39617, t39620, t39622, t39624, t39629, t39633, t39635, t39640, t39644)
}
