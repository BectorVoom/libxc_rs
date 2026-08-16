//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 615/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk615(t1564: f64, t25899: f64, t379: f64, t5674: f64, t25528: f64, t432: f64, t28: f64, t89: f64, t3103: f64, t5507: f64, t376: f64, t6516: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25901 = t1564 * t25899 * t379;
    let t25902 = t5674 * t25901;
    let t25904 = t25528 * t432;
    let t25905 = t28 * t25904;
    let t25906 = t89 * t25905;
    let t25908 = t5507 * t3103;
    let t25909 = t28 * t25908;
    let t25910 = t89 * t25909;
    let t25912 = t376 * t6516;
    (t25902, t25905, t25906, t25909, t25910, t25912)
}
