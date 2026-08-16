//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1425/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1425(t72: f64, t79: f64, t9240: f64, t2235: f64, t2251: f64, t2307: f64, t641: f64, t9342: f64, t1865: f64, t22523: f64, t22531: f64, t22537: f64, t22546: f64, t22554: f64, t605: f64, t6490: f64, t6492: f64, t6506: f64, t6510: f64, t83814: f64, t83817: f64, t83820: f64, t83822: f64, t83827: f64, t83830: f64) -> f64 {
    let t83832 = t72 * t79 * t9240;
    let t83835 = t2235 * t2251;
    let t83840 = t72 * t641 * t2307;
    let t83846 = t72 * t79 * t9342;
    let t83849 = -5.0_f64 * t83814 * t6492 + t605 * t83817 * t83820 + t83822 * t1865 / 3.0_f64 + t22537 * t6506 + t22537 * t6510 - 15.0_f64 * t83827 * t22546 + 35.0_f64 * t83830 * t83832 + t83835 * t1865 + 5.0_f64 / 2.0_f64 * t22523 * t22531 + 5.0_f64 / 2.0_f64 * t6490 * t83840 + 5.0_f64 / 2.0_f64 * t22554 * t22531 + 5.0_f64 / 6.0_f64 * t6490 * t83846;
    t83849
}
