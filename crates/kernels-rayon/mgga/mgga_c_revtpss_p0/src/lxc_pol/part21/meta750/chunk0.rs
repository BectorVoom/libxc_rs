//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2627/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2627(t47099: f64, t47101: f64, t13665: f64, t9575: f64, t47106: f64, t47110: f64, t47113: f64, t47119: f64, t47125: f64, t47127: f64, t40067: f64, t40072: f64, t47109: f64, t47116: f64, t47118: f64, t47122: f64, t47124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48311 = 0.17544670867903938621e1_f64 * t47099;
    let t48312 = 96.0_f64 * t47101;
    let t48313 = t13665 * t9575;
    let t48314 = 0.21687162600603479684e-1_f64 * t48313;
    let t48315 = 0.5848223622634646207e0_f64 * t47106;
    let t48316 = 0.10526802520742363173e2_f64 * t47110;
    let t48317 = 3.0_f64 * t47113;
    let t48318 = 0.97592231702715658578e-1_f64 * t47119;
    let t48319 = 0.14447919941302971324e1_f64 * t47125;
    let t48320 = 0.48796115851357829289e-1_f64 * t47127;
    let t48321 = -t48311 - t48312 - t48314 + t40067 - t40072 - t48315 - t47109 - t48316 + t48317 + t47116 - t47118 - t48318 + t47122 + t47124 + t48319 + t48320;
    (t48311, t48312, t48314, t48315, t48316, t48317, t48318, t48319, t48320, t48321)
}
