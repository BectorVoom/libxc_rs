//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1222/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1222(t3664: f64, t9294: f64, t11578: f64, t11579: f64, t1928: f64, t11577: f64, t11580: f64, t561: f64, t21643: f64, t26561: f64, t1743: f64, t26597: f64) -> (f64, f64, f64, f64, f64) {
    let t34353 = t3664 * t9294;
    let t34356 = t11578 * t11579 * t1928;
    let t34359 = t561 * t11577 * t11580;
    let t34361 = t26561 * t21643;
    let t34363 = t1743 * t26597;
    (t34353, t34356, t34359, t34361, t34363)
}
