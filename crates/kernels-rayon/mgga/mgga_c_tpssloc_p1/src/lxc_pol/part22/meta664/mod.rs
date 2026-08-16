//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2209;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2210;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2211;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2212;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta664(t17022: f64, t814: f64, t17100: f64, t225: f64, t17087: f64, t17060: f64, t17095: f64, t17098: f64, t10143: f64, t5660: f64, t17109: f64, t2752: f64, t2394: f64, t5678: f64, t17184: f64, t690: f64, t17179: f64, t17188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59347, t59466, t59498, t59503, t59519, t59537, t59564, t59584) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2209(t17022, t814, t17100, t225, t17087, t17060, t17095, t17098, t10143, t5660, t17109, t2752);
        let t59657 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2210(t2394, t5678);
        let t59663 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2211(t17184, t690);
        let t59665 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2212(t17179, t690);
        let t59680 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2213(t17188, t690);
    (t59347, t59466, t59498, t59503, t59519, t59537, t59564, t59584, t59657, t59663, t59665, t59680)
}
