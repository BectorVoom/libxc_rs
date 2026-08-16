//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2558;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2559;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2560;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta677(t14855: f64, t3411: f64, t14933: f64, t300: f64, t1166: f64, t3401: f64, t1155: f64, t3395: f64, t1695: f64, t11292: f64, t1164: f64, t3404: f64, t4857: f64, t11310: f64, t15225: f64, t51725: f64, t51399: f64, t51401: f64, t51404: f64, t51437: f64, t51439: f64, t51441: f64, t51443: f64, t51446: f64, t51449: f64, t51453: f64, t51456: f64, t51459: f64, t51463: f64, t51466: f64, t51470: f64, t51472: f64, t11433: f64, t14966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51806, t51809, t51811, t51814, t51818) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2558(t14855, t3411, t14933, t300, t1166, t3401, t1155, t3395, t1695, t11292, t1164, t3404, t4857);
        let (t51822, t51824, t51825) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2559(t11310, t300, t15225, t51811, t51725, t51399, t51401, t51404, t51437, t51439, t51806, t51809, t51814, t51818);
        let (t51826, t51831) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2560(t51441, t51443, t51446, t51449, t51453, t51456, t51459, t51463, t51466, t51470, t51472, t11433, t1164, t14966);
    (t51806, t51809, t51811, t51814, t51818, t51822, t51824, t51825, t51826, t51831)
}
