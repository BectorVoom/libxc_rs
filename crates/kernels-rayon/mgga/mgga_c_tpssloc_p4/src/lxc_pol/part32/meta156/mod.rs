//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta156 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk820;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk821;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk822;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk823;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk824;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta156(t225: f64, t4210: f64, t4217: f64, t228: f64, t68: f64, t1484: f64, t845: f64, t776: f64, t4119: f64, t824: f64, t1504: f64, t1506: f64, t230: f64, t822: f64, t825: f64, t232: f64, t819: f64, t820: f64, t4180: f64, t4181: f64, t829: f64, t120: f64, t2645: f64, t1516: f64, t2697: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4219, t4225, t4226, t4227, t4230, t4233) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk820(t225, t4210, t4217, t228, t68, t1484, t845, t776, t4119, t824, t1504, t1506, t230, t822, t825);
        let t4234 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk821(t232, t4233);
        let t4236 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk822(t4234, t819, t820);
        let t4240 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk823(t4180, t4181, t829);
        let t4250 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk824(t120, t1484, t2645, t829);
        let (t4253, t4255) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk825(t1516, t2697, t1484, t776);
    (t4219, t4225, t4226, t4227, t4230, t4233, t4234, t4236, t4240, t4250, t4253, t4255)
}
