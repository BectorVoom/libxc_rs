//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1198/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1198(t10610: f64, t10611: f64, t12056: f64, t10940: f64, t12086: f64, t11336: f64, t2850: f64, t3270: f64, t3269: f64, t3262: f64, t3465: f64, t40579: f64) -> (f64, f64, f64, f64) {
    let t41294 = 3.0_f64 / 2.0_f64 * t10610 * t12056 * t10611;
    let t41296 = t10940 * t12086 / 4.0_f64;
    let t41298 = t3270 * t11336 * t2850;
    let t41300 = t3269 * t41298 / 2.0_f64;
    let t41305 = 3.0_f64 / 4.0_f64 * t3262 * t3465 * t40579;
    (t41294, t41296, t41300, t41305)
}
