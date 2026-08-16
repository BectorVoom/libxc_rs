//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1275/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1275(t10041: f64, t96: f64, t36686: f64, t495: f64, t694: f64, t1679: f64, t1941: f64, t8040: f64, t11179: f64, t560: f64, t2254: f64, t28242: f64, t36747: f64, t36750: f64, t36753: f64, t36755: f64, t36756: f64, t5651: f64, t567: f64, t7297: f64, t8034: f64, t8048: f64, t8372: f64, t9089: f64, t9480: f64) -> f64 {
    let t42313 = t96 * t10041;
    let t42324 = t694 * t36686 * t495;
    let t42330 = t1679 * t8040 * t1941;
    let t42332 = t1679 * t11179 * t560;
    let t42337 = -6.0_f64 * t11179 * t7297 * t9089 + 6.0_f64 * t2254 * t36756 * t567 - 6.0_f64 * t28242 * t7297 * t8040 + 6.0_f64 * t5651 * t8034 * t8372 + 3.0_f64 * t567 * t8048 * t9480 + t36747 - t36750 + t36753 + t36755 + t42313 + 6.0_f64 * t42324 - t42330 - 2.0_f64 * t42332;
    t42337
}
