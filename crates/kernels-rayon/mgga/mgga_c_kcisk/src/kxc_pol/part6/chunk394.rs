//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 394/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk394(t2576: f64, t748: f64, t2527: f64, t747: f64, t746: f64, t1948: f64, t650: f64, t742: f64, t651: f64, t79: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2577 = t2576 * t748;
    let t2579 = t747 * t2527;
    let t2580 = t746 * t2579;
    let t2581 = t1948 * t2580;
    let t2583 = t742 * t650;
    let t2585 = 1.0_f64 / t651 / t2583;
    let t2586 = t2585 * t79;
    (t2577, t2579, t2580, t2581, t2585, t2586)
}
