//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1339/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1339(t24525: f64, t9239: f64, t39063: f64, t7245: f64, t2108: f64, t2110: f64, t22531: f64, t22537: f64, t22546: f64, t24514: f64, t24520: f64, t24526: f64, t605: f64, t607: f64, t7246: f64, t7256: f64, t7259: f64, t83745: f64, t83820: f64, t83822: f64, t83832: f64, t83835: f64, t83840: f64, t83846: f64) -> f64 {
    let t85480 = t9239 * t24525;
    let t85501 = t39063 * t7245;
    let t85504 = -15.0_f64 * t85480 * t22546 - 15.0_f64 * t24514 * t83745 + 5.0_f64 / 2.0_f64 * t24520 * t22531 + t83835 * t2110 + 5.0_f64 / 2.0_f64 * t24526 * t22531 + 5.0_f64 / 2.0_f64 * t7246 * t83840 + 5.0_f64 / 6.0_f64 * t7246 * t83846 + t605 * t607 * t2108 * t83820 + t83822 * t2110 / 3.0_f64 + t22537 * t7256 + t22537 * t7259 + 35.0_f64 * t85501 * t83832;
    t85504
}
