//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 363/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk363(t1666: f64, t567: f64, t1665: f64, t144: f64, t672: f64, t203: f64, t674: f64) -> (f64, f64, f64, f64) {
    let t1667 = t1666 * t567;
    let t1668 = t1665 * t1667;
    let t1671 = t672 * t144;
    let t1672 = t674 * t203;
    (t1667, t1668, t1671, t1672)
}
