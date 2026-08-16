//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk898;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk899;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk900;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk901;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk902;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk903;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta212(t2772: f64, t690: f64, t2777: f64, t2781: f64, t154: f64, t3061: f64, t10305: f64, t123: f64, t10309: f64, t2768: f64, t10316: f64, t882: f64, t10321: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t10558 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk898(t2772, t690);
        let t10560 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk899(t2777, t690);
        let t10562 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk900(t2781, t690);
        let (t10564, t10565, t10566) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk901(t154, t3061, t10305, t123);
        let (t10568, t10569) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk902(t10309, t2768, t123);
        let (t10571, t10572) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk903(t10316, t882, t123);
        let (t10574, t10575) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk904(t10321, t882, t123);
    (t10558, t10560, t10562, t10564, t10565, t10566, t10568, t10569, t10571, t10572, t10574, t10575)
}
