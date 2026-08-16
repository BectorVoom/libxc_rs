//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1288/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1288(t11588: f64, t27043: f64, t35175: f64, t3703: f64, t11418: f64, t3141: f64, t34863: f64, t505: f64, t128: f64, t567: f64, t5741: f64, t681: f64) -> (f64, f64, f64, f64) {
    let t35287 = t11588 * t27043;
    let t35289 = t35175 * t3703;
    let t35293 = t11418 * t3141 * t34863 * t505;
    let t35298 = t11418 * t5741 * t681 * t128 * t567;
    (t35287, t35289, t35293, t35298)
}
