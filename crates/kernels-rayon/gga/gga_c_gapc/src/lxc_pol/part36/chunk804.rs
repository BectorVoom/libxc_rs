//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 804/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk804(t129: f64, t7624: f64, t197: f64, t7626: f64, t2622: f64, t3336: f64, t7595: f64, t9435: f64, t2300: f64, t2636: f64, t3396: f64, t2979: f64) -> (f64, f64, f64, f64, f64) {
    let t9612 = t7624 * t129;
    let t9613 = t197 * t7626;
    let t9614 = t9612 * t9613;
    let t9616 = t3336 * t2622;
    let t9618 = t9435 * t7595;
    let t9620 = t2636 * t2300;
    let t9621 = t3396 * t9620;
    let t9623 = t7624 * t2979;
    (t9614, t9616, t9618, t9621, t9623)
}
