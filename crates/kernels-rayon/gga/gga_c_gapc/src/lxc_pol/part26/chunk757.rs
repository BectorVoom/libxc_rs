//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 757/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk757(t637: f64, t8900: f64, t1510: f64, t2982: f64, t3084: f64, t3131: f64, t3707: f64, t1030: f64, t4979: f64, t1631: f64, t190: f64, t1743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8901 = t8900 * t637;
    let t8903 = t2982 * t1510;
    let t8904 = t3084 * t8903;
    let t8906 = t3131 * t3707;
    let t8907 = t1030 * t8906;
    let t8908 = t8907 * t4979;
    let t8910 = t1631 * t190;
    let t8911 = t8910 * t3707;
    let t8912 = t1743 * t8911;
    (t8901, t8903, t8904, t8906, t8908, t8910, t8911, t8912)
}
