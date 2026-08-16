//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2535/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2535(t50834: f64, t51058: f64, t63291: f64, t63306: f64, t63308: f64, t63332: f64, t63334: f64, t63336: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t71144: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64) -> f64 {
    let t71371 = -2.0_f64 / 3.0_f64 * t63291 + 2.0_f64 / 9.0_f64 * t63306 - 10.0_f64 / 27.0_f64 * t63308 + t51058 - 28.0_f64 / 27.0_f64 * t50834 + 10.0_f64 / 9.0_f64 * t71124 - 8.0_f64 / 27.0_f64 * t63332 + 4.0_f64 / 9.0_f64 * t63334 - t63336 / 3.0_f64 - 4.0_f64 * t71130 + 40.0_f64 / 9.0_f64 * t71135 - 2.0_f64 / 9.0_f64 * t71140 + 2.0_f64 / 9.0_f64 * t71142 - 2.0_f64 / 3.0_f64 * t71144 - 10.0_f64 / 81.0_f64 * t71146 + t71150 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t71152 - t71154 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t71156 + 10.0_f64 / 9.0_f64 * t71160;
    t71371
}
