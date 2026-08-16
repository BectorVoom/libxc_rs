//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2265/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2265(t22480: f64, t7458: f64, t7461: f64, t9348: f64, t1774: f64, t22479: f64, t652: f64, t7468: f64, t15904: f64, t22574: f64, t33136: f64, t12734: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91706 = 2.0_f64 * t7458 * t22480;
    let t91708 = 2.0_f64 * t9348 * t7461;
    let t91713 = 2.0_f64 * t652 * t1774 * t22479;
    let t91715 = 2.0_f64 * t9348 * t7468;
    let t91718 = 6.0_f64 * t22574 * t33136 * t15904;
    let t91722 = 4.0_f64 * t12734 * t7468;
    (t91706, t91708, t91713, t91715, t91718, t91722)
}
