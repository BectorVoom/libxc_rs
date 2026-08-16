//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1377/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1377(t118414: f64, t118455: f64, t118467: f64, t118954: f64, t121258: f64, t121264: f64, t121271: f64, t121275: f64, t121279: f64, t22960: f64, t24191: f64, t25373: f64, t25375: f64, t26756: f64, t31442: f64, t33483: f64, t86716: f64, t86770: f64, t92319: f64) -> f64 {
    let t121283 = -3.0_f64 / 2.0_f64 * t92319 * t31442 - 3.0_f64 * t26756 * t86716 * t121258 - t121264 - 3.0_f64 / 2.0_f64 * t24191 * t118467 + t26756 * t86770 * t33483 - 3.0_f64 / 2.0_f64 * t24191 * t118455 + t121271 * t25375 + t26756 * t118414 + t26756 * t118954 + 3.0_f64 * t24191 * t25373 * t121275 - 3.0_f64 / 2.0_f64 * t24191 * t22960 * t121279;
    t121283
}
