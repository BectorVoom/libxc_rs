//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 762/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk762(t1610: f64, t1616: f64, t783: f64, t1234: f64, t133: f64, t1605: f64, t1604: f64, t20: f64, t489: f64, t524: f64, t525: f64, t2135: f64, t2294: f64) -> (f64, f64, f64, f64, f64) {
    let t6268 = t783 * t1610 * t1616;
    let t6271 = t1605 * t133 * t1234;
    let t6272 = t1604 * t6271;
    let t6291 = t489 * t20;
    let t6293 = t524 * t525 * t6291;
    let t6303 = t2294 * t2135;
    (t6268, t6271, t6272, t6293, t6303)
}
