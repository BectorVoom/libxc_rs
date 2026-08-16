//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 968/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk968(t240: f64, t29350: f64, t29352: f64, t29354: f64, t29356: f64, t29359: f64, t29362: f64, t29628: f64, t30119: f64, t567: f64, t564: f64, t2360: f64, t9295: f64) -> (f64, f64) {
    let t30121 = t240 * t30119 + t29350 - t29352 + t29354 - t29356 - t29359 + t29362 - t29628;
    let t30122 = t567 * t30121;
    let t30123 = t564 * t30122;
    let t30124 = t30123 / 16.0_f64;
    let t30125 = t2360 * t9295;
    (t30124, t30125)
}
