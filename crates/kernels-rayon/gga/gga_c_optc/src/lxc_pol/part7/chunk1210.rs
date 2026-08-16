//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1210/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1210(t24663: f64, t24664: f64, t24705: f64, t24976: f64, t140: f64, t7369: f64, t883: f64, t2661: f64, t24565: f64, t329: f64, t23548: f64, t7856: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24978 = t24663 + t24664 + t24705 + t24976;
    let t24985 = t883 * t7369 * t140;
    let t24986 = t2661 * t24985;
    let t24989 = t2661 * t24565;
    let t24995 = t329 * t24565;
    let t25001 = t7856 * t23548;
    (t24978, t24985, t24986, t24989, t24995, t25001)
}
