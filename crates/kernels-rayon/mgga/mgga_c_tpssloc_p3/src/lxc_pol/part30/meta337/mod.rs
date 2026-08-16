//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1368;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta337(t12985: f64, t2586: f64, t2570: f64, t67: f64, t792: f64, t12984: f64, t686: f64, t776: f64, t131: f64, t9558: f64, t205: f64, t1489: f64, t9541: f64, t4126: f64, t782: f64, t4130: f64, t2563: f64, t4138: f64, t4134: f64, t9546: f64, t118: f64, t4119: f64, t794: f64, t2576: f64, t225: f64, t4266: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12986, t13002, t13005, t13010) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1368(t12985, t2586, t2570, t67, t792, t12984, t686, t776, t131, t9558, t205, t1489, t9541);
        let (t13014, t13020, t13022, t13027, t13042) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1369(t4126, t782, t4130, t2563, t4138, t4134, t9546, t118, t4119, t794, t2576, t225, t4266);
    (t12986, t13002, t13005, t13010, t13014, t13020, t13022, t13027, t13042)
}
