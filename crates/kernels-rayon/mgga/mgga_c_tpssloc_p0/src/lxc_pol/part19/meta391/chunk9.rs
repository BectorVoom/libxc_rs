//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1480/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1480(t374: f64, t485: f64, t486: f64, t9697: f64, t1090: f64, t3493: f64, t11786: f64, t3490: f64, t11154: f64, t11784: f64, t1227: f64, t248: f64) -> (f64, f64, f64, f64) {
    let t45250 = 7.0_f64 / 31104.0_f64 * t485 * t374 * t9697 * t486;
    let t45251 = t1090 * t3493;
    let t45256 = t3490 * t11786;
    let t45260 = t1227 * t248 * t11784 * t11154;
    (t45250, t45251, t45256, t45260)
}
