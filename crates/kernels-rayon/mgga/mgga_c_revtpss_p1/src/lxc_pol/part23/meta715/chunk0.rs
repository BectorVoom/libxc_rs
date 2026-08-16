//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2474/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2474(t48255: f64, t47007: f64, t13597: f64, t2626: f64, t5571: f64, t9387: f64, t47013: f64, t13613: f64, t2619: f64, t9323: f64, t47019: f64, t47073: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48256 = 0.17544670867903938621e1_f64 * t48255;
    let t48259 = 144.0_f64 * t47007;
    let t48260 = t13597 * t2626;
    let t48261 = 0.35089341735807877242e1_f64 * t48260;
    let t48262 = t5571 * t9387;
    let t48266 = 48.0_f64 * t47013;
    let t48267 = t13613 * t2619;
    let t48268 = 0.73245789224026180216e-3_f64 * t48267;
    let t48269 = t5571 * t9323;
    let t48271 = 960.0_f64 * t47019;
    let t48279 = 8.0_f64 * t47073;
    (t48256, t48259, t48261, t48262, t48266, t48268, t48269, t48271, t48279)
}
