//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 926/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk926<F: Float>(t14332: F, t14352: F, t661: F, t2330: F, t3826: F, t1136: F, t9511: F, t1137: F, t1173: F, t14013: F, t14037: F, t2331: F, t2465: F, t2617: F, t263: F, t3683: F, t3827: F, t4003: F, t719: F, t771: F) -> F {
    let t14353 = t14332 + t14352;
    let t14354 = t661 * t14353;
    let t14358 = t2330 * t3826;
    let t14361 = t9511 * t1136;
    let t14365 = -t1137 * t2617 - t1173 * t2331 - t1173 * t2465 - t14354 * t263 - F::cast_from(2.0_f64) * t14358 * t263 - t14361 * t263 - F::cast_from(2.0_f64) * t3683 * t771 - F::cast_from(2.0_f64) * t3827 * t771 - F::cast_from(2.0_f64) * t4003 * t719 - F::cast_from(2.0_f64) * t14013 - F::cast_from(2.0_f64) * t14037;
    t14365
}
