//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1114/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1114(t80743: f64, t81281: f64, t81072: f64, t81074: f64, t80825: f64, t80847: f64, t80885: f64, t80899: f64, t80956: f64, t80970: f64, t81146: f64, t81153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84400 = 0.3244175520728446583e0_f64 * t80743;
    let t84423 = 0.19739208802178717238e0_f64 * t81281;
    let t84480 = 0.55440370401180965083e0_f64 * t81072;
    let t84481 = 0.3244175520728446583e0_f64 * t81074;
    let t84514 = 0.2034786907144675699e0_f64 * t80825;
    let t84520 = 455.0_f64 / 648.0_f64 * t80847;
    let t84533 = 0.67287926823567318088e-4_f64 * t80885;
    let t84536 = 595.0_f64 / 2592.0_f64 * t80899;
    let t84555 = 0.13958506597733353653e-1_f64 * t80956;
    let t84558 = 0.87474304870637513515e-3_f64 * t80970;
    let t84595 = 0.27415567780803773942e-2_f64 * t81146;
    let t84597 = 0.19739208802178717238e0_f64 * t81153;
    (t84400, t84423, t84480, t84481, t84514, t84520, t84533, t84536, t84555, t84558, t84595, t84597)
}
