//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1782/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1782(t13263: f64, t4282: f64, t2633: f64, t9632: f64, t2732: f64, t4234: f64, t2679: f64, t4295: f64, t1519: f64, t2627: f64, t10076: f64, t1510: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13398 = t4282 * t13263;
    let t13401 = t4282 * t2633;
    let t13404 = t4282 * t9632;
    let t13407 = t2732 * t4234;
    let t13414 = t4295 * t2679;
    let t13416 = t2627 * t1519;
    let t13417 = t13416 * t2633;
    let t13423 = t10076 * t1510;
    (t13398, t13401, t13404, t13407, t13414, t13417, t13423)
}
