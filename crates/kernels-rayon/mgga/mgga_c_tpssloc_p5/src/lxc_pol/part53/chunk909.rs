//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 909/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk909(t214: f64, t33383: f64, t1880: f64, t6571: f64, t7841: f64, t6553: f64, t31366: f64, t7479: f64, t6552: f64, t7488: f64, t225: f64, t258: f64, t7823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33384 = t214 * t33383;
    let t33385 = t1880 * t33384;
    let t33408 = t6571 * t7841;
    let t33409 = t6553 * t33408;
    let t33410 = t1880 * t33409;
    let t33419 = t31366 * t7479;
    let t33420 = t6552 * t33419;
    let t33422 = t31366 * t7488;
    let t33423 = t1880 * t33422;
    let t33428 = t7823 * t225 * t258;
    (t33384, t33385, t33408, t33409, t33410, t33419, t33420, t33422, t33423, t33428)
}
