//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta498 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2116;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2117;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2118;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2119;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2120;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta498(t10143: f64, t5664: f64, t12895: f64, t13121: f64, t1484: f64, t16697: f64, t16699: f64, t16700: f64, t16703: f64, t16705: f64, t16707: f64, t16708: f64, t16709: f64, t16712: f64, t16715: f64, t16719: f64, t1877: f64, t193: f64, t2522: f64, t262: f64, t5527: f64, t776: f64, t868: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64, t16610: f64, t16666: f64, t17119: f64, t1534: f64, t2: f64, t584: f64, t5678: f64, t690: f64, t10216: f64, t5392: f64, t607: f64, t10564: f64, t123: f64, t10277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17120, t17131) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2116(t10143, t5664, t12895, t13121, t1484, t16697, t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16712, t16715, t16719, t1877, t193, t2522, t262, t5527, t776, t868, t9853, t9859, t9894, t9907, t9921);
        let t17133 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2117(t16610, t16666, t17119, t17131);
        let (t17139, t17141, t17149) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2118(t1534, t2, t584, t5678, t690);
        let (t17151, t17152) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2119(t10216, t5392, t607);
        let (t17153, t17154) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2120(t10564, t17152, t123);
        let (t17156, t17157) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2121(t10277, t5392, t607);
    (t17120, t17133, t17139, t17141, t17149, t17151, t17152, t17153, t17154, t17156, t17157)
}
