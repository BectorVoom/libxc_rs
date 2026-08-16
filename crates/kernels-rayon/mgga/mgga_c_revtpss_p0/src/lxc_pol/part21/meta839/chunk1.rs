//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3147/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3147(t3362: f64, t462: f64, t51959: f64, t52011: f64, t44348: f64, t44919: f64, t12327: f64, t3391: f64, t5079: f64, t12331: f64, t1134: f64, t16926: f64, t3390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58027 = t462 * t3362;
    let t58029 = t52011 * t58027 * t51959;
    let t58032 = t52011 * t44348 * t51959;
    let t58035 = t52011 * t44919 * t51959;
    let t58038 = t12327 * t5079 * t3391;
    let t58041 = t12331 * t5079 * t3391;
    let t58044 = t3390 * t16926 * t1134;
    (t58029, t58032, t58035, t58038, t58041, t58044)
}
