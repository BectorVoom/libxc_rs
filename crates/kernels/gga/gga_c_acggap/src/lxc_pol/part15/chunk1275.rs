//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1275/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1275<F: Float>(t10041: F, t96: F, t36686: F, t495: F, t694: F, t1679: F, t1941: F, t8040: F, t11179: F, t560: F, t2254: F, t28242: F, t36747: F, t36750: F, t36753: F, t36755: F, t36756: F, t5651: F, t567: F, t7297: F, t8034: F, t8048: F, t8372: F, t9089: F, t9480: F) -> F {
    let t42313 = t96 * t10041;
    let t42324 = t694 * t36686 * t495;
    let t42330 = t1679 * t8040 * t1941;
    let t42332 = t1679 * t11179 * t560;
    let t42337 = -F::new(6.0) * t11179 * t7297 * t9089 + F::new(6.0) * t2254 * t36756 * t567 - F::new(6.0) * t28242 * t7297 * t8040 + F::new(6.0) * t5651 * t8034 * t8372 + F::new(3.0) * t567 * t8048 * t9480 + t36747 - t36750 + t36753 + t36755 + t42313 + F::new(6.0) * t42324 - t42330 - F::new(2.0) * t42332;
    t42337
}
