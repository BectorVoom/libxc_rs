//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 811/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk811(t1196: f64, t2250: f64, t974: f64, t1176: f64, t3247: f64, t2244: f64, t3242: f64, t3439: f64, t225: f64, t3481: f64, t68: f64, t484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3551 = t1196 * t2250;
    let t3552 = t974 * t3551;
    let t3555 = t1176 * t3247;
    let t3556 = t3555 * t2244;
    let t3557 = t974 * t3556;
    let t3560 = t3439 * t3242;
    let t3561 = t3560 * t2244;
    let t3562 = t974 * t3561;
    let t3565 = t3481 * t225;
    let t3566 = t3565 * t68;
    let t3567 = t3566 * t484;
    (t3551, t3552, t3556, t3557, t3561, t3562, t3565, t3566, t3567)
}
