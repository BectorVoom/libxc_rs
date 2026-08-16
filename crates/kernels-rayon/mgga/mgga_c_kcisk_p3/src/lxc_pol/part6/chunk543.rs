//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 543/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk543(t2637: f64, t4998: f64, t2013: f64, t2643: f64, t4419: f64, t782: f64, t2642: f64, t5507: f64, t1993: f64, t2618: f64, t2041: f64, t2656: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7602 = t4998 * t2637;
    let t7603 = t2013 * t7602;
    let t7624 = t4419 * t2643;
    let t7625 = t782 * t7624;
    let t7632 = t5507 * t2642;
    let t7648 = t2618 * t1993;
    let t7656 = t2656 * t2041;
    (t7602, t7603, t7624, t7625, t7632, t7648, t7656)
}
