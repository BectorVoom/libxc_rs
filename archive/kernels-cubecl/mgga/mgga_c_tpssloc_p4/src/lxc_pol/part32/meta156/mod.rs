//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk820;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk821;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk822;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk823;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk824;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta156<F: Float>(t225: F, t4210: F, t4217: F, t228: F, t68: F, t1484: F, t845: F, t776: F, t4119: F, t824: F, t1504: F, t1506: F, t230: F, t822: F, t825: F, t232: F, t819: F, t820: F, t4180: F, t4181: F, t829: F, t120: F, t2645: F, t1516: F, t2697: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4219, t4225, t4226, t4227, t4230, t4233) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk820::<F>(t225, t4210, t4217, t228, t68, t1484, t845, t776, t4119, t824, t1504, t1506, t230, t822, t825);
        let t4234 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk821::<F>(t232, t4233);
        let t4236 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk822::<F>(t4234, t819, t820);
        let t4240 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk823::<F>(t4180, t4181, t829);
        let t4250 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk824::<F>(t120, t1484, t2645, t829);
        let (t4253, t4255) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk825::<F>(t1516, t2697, t1484, t776);
    (t4219, t4225, t4226, t4227, t4230, t4233, t4234, t4236, t4240, t4250, t4253, t4255)
}
