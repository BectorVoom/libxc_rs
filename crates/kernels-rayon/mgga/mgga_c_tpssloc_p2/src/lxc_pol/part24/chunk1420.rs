//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1420/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1420(t22530: f64, t645: f64, t72: f64, t1864: f64, t2307: f64, t1863: f64, t22522: f64, t9239: f64, t2241: f64, t641: f64, t608: f64, t9228: f64) -> (f64, f64, f64, f64, f64) {
    let t83734 = t72 * t22530 * t645;
    let t83737 = t1864 * t2307;
    let t83738 = t1863 * t83737;
    let t83741 = t9239 * t22522;
    let t83745 = t72 * t641 * t2241;
    let t83748 = t9228 * t608;
    (t83734, t83738, t83741, t83745, t83748)
}
