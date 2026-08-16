//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1017/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1017(t11570: f64, t2244: f64, t11569: f64, t3448: f64, t3469: f64, t3451: f64, t2250: f64, t3450: f64, t3449: f64, t3247: f64, t460: f64, t1176: f64, t134: f64) -> (f64, f64, f64, f64, f64) {
    let t11571 = t11570 * t2244;
    let t11572 = t11569 * t11571;
    let t11575 = t3448 * t3469;
    let t11576 = t11575 * t3451;
    let t11579 = t3450 * t2250;
    let t11580 = t3449 * t11579;
    let t11583 = t460 * t3247;
    let t11584 = t11583 * t2244;
    let t11585 = t3449 * t11584;
    let t11588 = t134 * t1176;
    (t11572, t11576, t11580, t11585, t11588)
}
