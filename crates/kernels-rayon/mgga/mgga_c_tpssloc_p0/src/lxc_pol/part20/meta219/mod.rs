//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1287;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1288;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta219(t3: f64, t5363: f64, t112: f64, t1851: f64, t1458: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t4072: f64, t577: f64, t154: f64, t781: f64, t202: f64, t243: f64, t2229: f64, t61: f64, t119: f64, t212: f64, t252: f64, t828: f64, t343: f64, t984: f64, t3034: f64, t334: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5364, t5371, t5376, t5381, t6546) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1287(t3, t5363, t112, t1851, t1458, t671, t1401, t3938, t3941, t4072, t577, t154, t781);
        let (t6589, t6597, t6600, t6647, t6733, t6739) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1288(t202, t243, t2229, t61, t119, t212, t252, t828, t343, t984, t3034, t334);
    (t5364, t5371, t5376, t5381, t6546, t6589, t6597, t6600, t6647, t6733, t6739)
}
