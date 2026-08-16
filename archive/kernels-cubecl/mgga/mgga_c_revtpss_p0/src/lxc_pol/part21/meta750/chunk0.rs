//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2627/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2627<F: Float>(t47099: F, t47101: F, t13665: F, t9575: F, t47106: F, t47110: F, t47113: F, t47119: F, t47125: F, t47127: F, t40067: F, t40072: F, t47109: F, t47116: F, t47118: F, t47122: F, t47124: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t48311 = F::cast_from(0.17544670867903938621e1_f64) * t47099;
    let t48312 = F::cast_from(96.0_f64) * t47101;
    let t48313 = t13665 * t9575;
    let t48314 = F::cast_from(0.21687162600603479684e-1_f64) * t48313;
    let t48315 = F::cast_from(0.5848223622634646207e0_f64) * t47106;
    let t48316 = F::cast_from(0.10526802520742363173e2_f64) * t47110;
    let t48317 = F::cast_from(3.0_f64) * t47113;
    let t48318 = F::cast_from(0.97592231702715658578e-1_f64) * t47119;
    let t48319 = F::cast_from(0.14447919941302971324e1_f64) * t47125;
    let t48320 = F::cast_from(0.48796115851357829289e-1_f64) * t47127;
    let t48321 = -t48311 - t48312 - t48314 + t40067 - t40072 - t48315 - t47109 - t48316 + t48317 + t47116 - t47118 - t48318 + t47122 + t47124 + t48319 + t48320;
    (t48311, t48312, t48314, t48315, t48316, t48317, t48318, t48319, t48320, t48321)
}
