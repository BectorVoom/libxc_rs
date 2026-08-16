//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 710/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk710(t1947: f64, t8426: f64, t473: f64, t1510: f64, t493: f64, t2928: f64, t1273: f64, t991: f64, t1007: f64, t1484: f64, t1492: f64, t433: f64, t463: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8427 = t8426 * t1947;
    let t8428 = t473 * t8427;
    let t8430 = t493 * t1510;
    let t8431 = t2928 * t8430;
    let t8433 = t1273 * t991;
    let t8435 = t1484 * t1007;
    let t8437 = t1492 * t1007;
    let t8442 = t463 * t433;
    (t8428, t8431, t8433, t8435, t8437, t8442)
}
