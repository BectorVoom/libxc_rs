//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1146/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1146(t16988: f64, t2641: f64, t17060: f64, t24: f64, t862: f64, t17064: f64, t17031: f64, t24583: f64, t16975: f64, t2640: f64, t32131: f64, t17045: f64, t2669: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49850 = t2641 * t16988;
    let t49860 = t862 * t24 * t17060;
    let t49865 = t862 * t24 * t17064;
    let t49869 = t24583 * t17031;
    let t49882 = t2640 * t32131 * t16975;
    let t49896 = t2669 * t17045;
    (t49850, t49860, t49865, t49869, t49882, t49896)
}
