//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1491/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1491(t626: f64, t9412: f64, t106: f64, t9364: f64, t2332: f64, t2358: f64, t2248: f64, t35761: f64, t2350: f64, t2354: f64, t39108: f64, t35577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45432 = t626 * t9412;
    let t45435 = 1.0_f64 / t9364 / t106;
    let t45436 = t2332 * t2332;
    let t45444 = t2358 * t2358;
    let t45453 = t2248 * t2248;
    let t45460 = 1.0_f64 / t35761;
    let t45461 = t2350 * t2350;
    let t45469 = t2354 * t2354;
    let t45482 = 12.0_f64 * t39108;
    let t45496 = 1.0_f64 / t35577;
    (t45432, t45435, t45436, t45444, t45453, t45460, t45461, t45469, t45482, t45496)
}
