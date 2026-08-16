//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2661/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2661(t19297: f64, t604: f64, t4021: f64, t12571: f64, t12585: f64, t12588: f64, t19299: f64, t19310: f64, t19318: f64, t19445: f64, t2235: f64, t2240: f64, t2241: f64, t2307: f64, t39054: f64, t39063: f64, t3958: f64, t46104: f64, t5389: f64, t5445: f64, t55631: f64, t55673: f64, t55709: f64, t55875: f64, t605: f64, t645: f64, t9228: f64, t9231: f64, t9239: f64) -> f64 {
    let t55880 = t19297 * t604;
    let t55885 = t4021 * t4021;
    let t55888 = 40.0_f64 * t9231 * t19318 - 240.0_f64 * t39054 * t19310 - 120.0_f64 * t9239 * t5389 * t2307 - 8.0_f64 * t2235 * t19445 + 80.0_f64 * t46104 * t3958 + 80.0_f64 * t12571 * t12585 + 40.0_f64 * t12571 * t12588 + 840.0_f64 * t39063 * t5389 * t2241 - 4.0_f64 * t9228 * t5445 - 4.0_f64 * t605 * (t55631 + t55673 + t55709 + t55875) - 8.0_f64 * t55880 * t645 - 4.0_f64 * t19299 * t2307 + 40.0_f64 * t2240 * t55885;
    t55888
}
