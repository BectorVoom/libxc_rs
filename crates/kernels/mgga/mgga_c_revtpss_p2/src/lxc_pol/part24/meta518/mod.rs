//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1539;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1540;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1541;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta518<F: Float>(t24042: F, t994: F, t23959: F, t378: F, t4746: F, t6343: F, t79862: F, t1647: F, t1678: F, t6235: F, t342: F, t25026: F, t3801: F, t1130: F, t24466: F, t24237: F, t689: F, t24245: F, t24229: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t80810, t80833, t80901, t80921, t80983, t80992, t81052, t81139) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1539::<F>(t24042, t994, t23959, t378, t4746, t6343, t79862, t1647, t1678, t6235, t342, t25026, t3801);
        let (t81146, t81156) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1540::<F>(t1130, t24466, t24237, t689);
        let t81158 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1541::<F>(t24245, t689);
        let t81230 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1542::<F>(t24229, t689);
    (t80810, t80833, t80901, t80921, t80983, t80992, t81052, t81139, t81146, t81156, t81158, t81230)
}
