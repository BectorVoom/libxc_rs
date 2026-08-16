//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1144/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1144(t204: f64, t2368: f64, t2459: f64, t2462: f64, t2471: f64, t2472: f64, t2476: f64, t2480: f64, t2490: f64, t2494: f64, t2495: f64, t2505: f64, t2509: f64, t2513: f64, t268: f64, t39373: f64, t39389: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t676: f64, t746: f64, t9489: f64, t9729: f64, t9734: f64, t9739: f64, t9755: f64, t9759: f64, t9766: f64, t9803: f64, t9810: f64, t9814: f64) -> f64 {
    let t39749 = 0.12842595503380418954e1_f64 * t268 * t204 * t2509 * t2513 - 0.21687162600603479684e-1_f64 * t268 * t2490 * t9766 - 0.38025319932552508021e2_f64 * t268 * t676 * t9489 * t9759 + 0.43374325201206959368e-1_f64 * t268 * t9803 * t2505 - 0.27397333333333333333e0_f64 * t268 * t204 * t2459 * t2462 - 0.14171548179536397724e3_f64 * t268 * t676 * t9729 * t9734 - 0.86748650402413918736e-1_f64 * t268 * t204 * t2368 * t2495 - 0.1301229756036208781e0_f64 * t268 * t9810 * t9755 + 0.13698666666666666666e0_f64 * t268 * t9814 * t2472 + 0.44060335298551228073e1_f64 * t268 * t204 * t2476 * t2480 - t39373 + t39397 + t39400 - t39408 - t39411 - 0.11579025239058625248e4_f64 * t9739 * t2480 * t2471 - 0.35089341735807877242e1_f64 * t2494 * t39389 * t746;
    t39749
}
