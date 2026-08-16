//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1302;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1303;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta358(t10638: f64, t231: f64, t268: f64, t2798: f64, t675: f64, t2645: f64, t837: f64, t2782: f64, t2797: f64, t10115: f64, t883: f64, t2482: f64, t2811: f64, t39588: f64, t686: f64, t72: f64, t2710: f64, t2793: f64, t39494: f64, t2804: f64, t874: f64, t9288: f64, t10535: f64, t281: f64, t68: f64, t211: f64, t9644: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39617, t39620, t39622, t39624, t39629) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1302(t10638, t231, t268, t2798, t675, t2645, t837, t2782, t2797, t10115, t883, t2482, t2811, t39588, t686, t72);
        let (t39633, t39635, t39640, t39643) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1303(t2710, t2793, t39494, t2804, t874, t9288, t10535, t231, t2645, t281, t68, t211, t9644);
        let t39644 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1304(t209, t39643);
    (t39617, t39620, t39622, t39624, t39629, t39633, t39635, t39640, t39644)
}
