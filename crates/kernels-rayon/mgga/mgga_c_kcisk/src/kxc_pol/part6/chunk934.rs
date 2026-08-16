//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 934/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk934(t29564: f64, t741: f64, t2591: f64, t9062: f64, t2560: f64, t9036: f64, t28314: f64, t642: f64, t735: f64, t734: f64, t2567: f64, t9029: f64) -> (f64, f64, f64, f64, f64) {
    let t29565 = t741 * t29564;
    let t29567 = t9062 * t2591;
    let t29569 = t2560 * t9036;
    let t29571 = t642 * t28314;
    let t29572 = t735 * t29571;
    let t29573 = t734 * t29572;
    let t29575 = t2567 * t9029;
    (t29565, t29567, t29569, t29573, t29575)
}
