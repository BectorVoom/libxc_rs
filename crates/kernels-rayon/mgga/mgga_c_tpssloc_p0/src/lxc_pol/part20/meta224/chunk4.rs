//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1303/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1303(t5: f64, t2235: f64, t2240: f64, t2241: f64, t2307: f64, t605: f64, t645: f64, t86: f64, t9226: f64, t9228: f64, t9231: f64, t9239: f64, t9240: f64, t9243: f64, t9342: f64) -> f64 {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t9346 = piecewise3(t8, 0.0_f64, -12.0_f64 * t2235 * t2307 + 60.0_f64 * t2240 * t9243 + 60.0_f64 * t2241 * t9231 - 4.0_f64 * t605 * t9342 - 12.0_f64 * t645 * t9228 + t86 * t9226 - 120.0_f64 * t9239 * t9240);
    t9346
}
