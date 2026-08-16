//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta129 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk845;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk846;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk847;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk848;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk849;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta129(t1013: f64, t361: f64, t363: f64, t3037: f64, t3033: f64, t360: f64, t3040: f64, t1021: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t3127, t3128) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk845(t1013, t361, t363);
        let t3129 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk846(t3037, t3128);
        let t3130 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk847(t3033, t3129);
        let t3131 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk848(t360);
        let t3132 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk849(t3040, t3131);
        let t3134 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk850(t1021, t248, t3132);
    (t3127, t3128, t3129, t3130, t3131, t3132, t3134)
}
