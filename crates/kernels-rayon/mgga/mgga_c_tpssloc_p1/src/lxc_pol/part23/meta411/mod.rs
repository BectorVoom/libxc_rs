//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1227;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta411(t1174: f64, t6177: f64, t698: f64, t3545: f64, t6109: f64, t15753: f64, t4889: f64, t1244: f64, t3068: f64, t478: f64, t6163: f64, t6183: f64, t22430: f64, t580: f64, t111: f64, t20292: f64, t172: f64, t20742: f64, t763: f64, t21066: f64, t870: f64, t2752: f64, t20767: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66057, t66500, t66545, t66622, t66668) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1227(t1174, t6177, t698, t3545, t6109, t15753, t4889, t1244, t3068, t478, t6163, t6183);
        let (t67000, t67001, t67099, t67112, t67154, t67159) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1228(t22430, t580, t111, t20292, t172, t20742, t763, t21066, t870, t2752, t20767, t751);
    (t66057, t66500, t66545, t66622, t66668, t67000, t67001, t67099, t67112, t67154, t67159)
}
