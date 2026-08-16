//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1031/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1031(t4290: f64, t808: f64, t13380: f64, t4182: f64, t68: f64, t9971: f64, t226: f64, t13263: f64, t4282: f64, t2633: f64, t9632: f64, t2732: f64, t4234: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13390 = t808 * t4290;
    let t13393 = t13380 * t4182;
    let t13396 = t68 * t9971;
    let t13397 = t226 * t13396;
    let t13398 = t4282 * t13263;
    let t13401 = t4282 * t2633;
    let t13404 = t4282 * t9632;
    let t13407 = t2732 * t4234;
    (t13390, t13393, t13397, t13398, t13401, t13404, t13407)
}
