//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 527/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk527(t137: f64, t2584: f64, t154: f64, t754: f64, t804: f64, t809: f64, t805: f64, t812: f64, t152: f64, t2489: f64, t2491: f64, t774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2585 = t2584 * t137;
    let t2586 = t2585 * t154;
    let t2588 = t804 * t754;
    let t2589 = t2588 * t809;
    let t2591 = t805 * t812;
    let t2593 = t152 * t2489;
    let t2594 = t154 * t2491;
    let t2595 = t2593 * t2594;
    let t2597 = t812 * t774;
    (t2585, t2586, t2588, t2589, t2591, t2593, t2594, t2595, t2597)
}
