//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk855;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta212(t13004: f64, t205: f64, t1489: f64, t9541: f64, t4126: f64, t782: f64, t4134: f64, t9546: f64, t1496: f64, t2528: f64, t4199: f64, t2663: f64, t4211: f64, t2535: f64, t1471: f64, t32: f64, t118: f64, t1474: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13005, t13010, t13012, t13022, t13087, t13107, t13109) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk855(t13004, t205, t1489, t9541, t4126, t782, t4134, t9546, t1496, t2528, t4199, t2663, t4211);
        let (t13113, t13115, t13123) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk856(t2535, t4199, t1471, t32, t118, t1474);
    (t13005, t13010, t13012, t13022, t13087, t13107, t13109, t13113, t13115, t13123)
}
