//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 853/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk853(t1628: f64, t2833: f64, t2815: f64, t1589: f64, t2792: f64, t447: f64, t7892: f64, t6964: f64, t1: f64, t7887: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8393 = t1628 * t2833;
    let t8398 = t1628 * t2815;
    let t8403 = t1589 * t2792;
    let t8406 = t7892 * t447;
    let t8407 = t6964 * t8406;
    let t8410 = t7887 * t1;
    (t8393, t8398, t8403, t8406, t8407, t8410)
}
