//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1010/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1010(t11597: f64, t2993: f64, t3001: f64, t1030: f64, t3008: f64, t11356: f64, t9256: f64, t932: f64, t996: f64, t3723: f64, t787: f64, t876: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11598 = t2993 * t11597;
    let t11599 = t11598 * t3001;
    let t11601 = t1030 * t11597;
    let t11602 = t11601 * t3008;
    let t11604 = t2993 * t11356;
    let t11605 = t11604 * t9256;
    let t11612 = t996 * t932;
    let t11613 = t3723 * t787;
    let t11614 = t11612 * t11613;
    let t11616 = t3723 * t876;
    (t11598, t11599, t11601, t11602, t11604, t11605, t11612, t11613, t11614, t11616)
}
