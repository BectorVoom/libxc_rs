//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2188/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2188(t108530: f64, t108551: f64, t108564: f64, t108580: f64, t108589: f64, t108596: f64, t108613: f64, t108631: f64, t108502: f64, t14230: f64, t1903: f64, t213: f64, t22395: f64, t225: f64, t25930: f64, t25931: f64, t27868: f64, t27980: f64, t561: f64, t7279: f64, t75016: f64, t94884: f64, t98333: f64, t98338: f64, t98358: f64, t98360: f64, t98368: f64, t98372: f64, t98376: f64, t98379: f64) -> (f64, f64) {
    let t108634 = t108530 + t108551 + t108564 + t108580 + t108589 + t108596 + t108613 + t108631;
    let t108651 = 0.17347256376410398924e1_f64 * t25930 * t27980 * t108502 + 0.13009920719177044025e-1_f64 * t94884 + 0.65854491829355115987e0_f64 * t213 * t108634 * t225 * t561 + 0.4336814094102599731e0_f64 * t27868 * t25931 * t75016 - 0.68540937416128198416e-1_f64 * t98333 - 0.68540937416128198419e-2_f64 * t98338 - t98358 - t98360 + 0.26341796731742046394e1_f64 * t7279 * t22395 + 0.34694512752820797848e1_f64 * t25930 * t27980 * t1903 * t14230 - t98368 - 0.26019841438354088051e-1_f64 * t98372 + t98376 + t98379;
    (t108634, t108651)
}
