//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1015/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1015(t1347: f64, t7605: f64, t1980: f64, t35383: f64, t7458: f64, t31773: f64, t8634: f64, t2288: f64, t4210: f64, t15386: f64, t31057: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35678 = t7605 * t1347;
    let t35682 = t1980 * t7458 * t35383;
    let t35685 = t31773 * t8634;
    let t35700 = t2288 * t4210;
    let t35702 = t31057 * t15386 * t35700;
    let t35709 = t7614 * t1347;
    (t35678, t35682, t35685, t35700, t35702, t35709)
}
