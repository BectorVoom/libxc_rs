//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 393/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk393(t2567: f64, t719: f64, t734: f64, t2441: f64, t642: f64, t735: f64, t2507: f64, t716: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2568 = t2567 * t719;
    let t2569 = t734 * t2568;
    let t2571 = t642 * t2441;
    let t2572 = t735 * t2571;
    let t2573 = t734 * t2572;
    let t2575 = t2507 * t716;
    let t2576 = t2575 * t740;
    (t2568, t2569, t2571, t2572, t2573, t2575, t2576)
}
