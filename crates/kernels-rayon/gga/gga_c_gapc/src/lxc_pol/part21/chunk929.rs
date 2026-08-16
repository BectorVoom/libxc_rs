//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 929/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk929(t11417: f64, t116: f64, t128: f64, t1672: f64, t1906: f64, t515: f64, t644: f64, t19: f64, t169: f64, t3665: f64, t8652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11418 = t116 * t11417;
    let t11420 = t1906 * t1672 * t128;
    let t11421 = t11418 * t11420;
    let t11423 = t515 * t644;
    let t11424 = t11423 * t19;
    let t11425 = t169 * t11424;
    let t11426 = t11425 * t3665;
    let t11428 = 1.0_f64 / t8652;
    (t11418, t11420, t11421, t11423, t11424, t11425, t11426, t11428)
}
