//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk920;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk921;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta221(t2884: f64, t307: f64, t302: f64, t10743: f64, t2888: f64, t10294: f64, t10544: f64, t10296: f64, t10298: f64, t10300: f64, t10302: f64, t10307: f64, t10314: f64, t10320: f64, t10323: f64, t10530: f64, t10538: f64, t10547: f64, t10550: f64, t10311: f64, t10318: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10566: f64, t10569: f64, t10572: f64, t10575: f64, t10589: f64, t10591: f64, t10597: f64, t10600: f64, t932: f64, t922: f64, t2887: f64, t310: f64, t2791: f64, t888: f64, t2794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10770, t10771, t10772, t10789) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk920(t2884, t307, t302, t10743, t2888, t10294, t10544, t10296, t10298, t10300, t10302, t10307, t10314, t10320, t10323, t10530, t10538, t10547, t10550);
        let t10804 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk921(t10311, t10318, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575, t10589, t10591, t10597, t10600);
        let (t10805, t10806, t10810, t10811, t10813, t10814, t10817, t10819) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk922(t10789, t10804, t932, t2884, t922, t302, t2887, t310, t10743, t2791, t888, t2794);
    (t10770, t10771, t10772, t10805, t10806, t10810, t10811, t10813, t10814, t10817, t10819)
}
