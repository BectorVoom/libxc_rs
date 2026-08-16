//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1495;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta257(t3: f64, t6470: f64, t1401: f64, t1458: f64, t3941: f64, t5371: f64, t5456: f64, t5493: f64, t577: f64, t641: f64, t71: f64, t154: f64, t781: f64, t202: f64, t243: f64, t2229: f64, t61: f64, t119: f64, t212: f64, t343: f64, t984: f64, t3034: f64, t334: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6471, t6483, t6509, t6546) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1495(t3, t6470, t1401, t1458, t3941, t5371, t5456, t5493, t577, t641, t71, t154, t781);
        let (t6589, t6597, t6600, t6733, t6739) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1496(t202, t243, t2229, t61, t119, t212, t343, t984, t3034, t334);
    (t6471, t6483, t6509, t6546, t6589, t6597, t6600, t6733, t6739)
}
