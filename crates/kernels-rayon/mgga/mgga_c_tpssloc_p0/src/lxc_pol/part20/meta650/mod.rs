//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2390;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2391;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2392;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta650(t13634: f64, t49039: f64, t13615: f64, t2798: f64, t896: f64, t2815: f64, t13623: f64, t2807: f64, t10588: f64, t4378: f64, t13629: f64, t48981: f64, t894: f64, t42086: f64, t42087: f64, t47781: f64, t47785: f64, t47787: f64, t48907: f64, t48920: f64, t48934: f64, t48990: f64, t49004: f64, t49026: f64, t49042: f64, t893: f64, t913: f64, t14388: f64, t2836: f64, t2842: f64, t10704: f64, t4395: f64, t10702: f64, t2793: f64, t10524: f64, t10603: f64, t10717: f64, t10724: f64, t10734: f64, t10756: f64, t10765: f64, t14271: f64, t14276: f64, t14337: f64, t14369: f64, t14459: f64, t14466: f64, t1580: f64, t2906: f64, t2924: f64, t2930: f64, t41826: f64, t41981: f64, t42111: f64, t42113: f64, t42123: f64, t4416: f64, t4438: f64, t4475: f64, t48883: f64, t48890: f64, t950: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49043, t49049, t49052, t49054, t49056, t49058, t49060) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2390(t13634, t49039, t13615, t2798, t896, t2815, t13623, t2807, t10588, t4378, t13629, t48981, t894);
        let t49062 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2391(t42086, t42087, t47781, t47785, t47787, t49043, t49049, t49052, t49054, t49056, t49058, t49060);
        let (t49068, t49071) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2392(t48907, t48920, t48934, t48990, t49004, t49026, t49042, t49062, t893, t913, t14388, t2836, t2842);
        let (t49075, t49076) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2393(t10704, t4395, t10702, t2793, t10524, t10603, t10717, t10724, t10734, t10756, t10765, t14271, t14276, t14337, t14369, t14459, t14466, t1580, t2906, t2924, t2930, t41826, t41981, t42111, t42113, t42123, t4416, t4438, t4475, t48883, t48890, t49068, t49071, t950);
    (t49043, t49049, t49052, t49054, t49056, t49058, t49060, t49068, t49071, t49075, t49076)
}
