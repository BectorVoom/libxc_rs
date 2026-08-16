//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3818/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3818(t48255: f64, t46999: f64, t47005: f64, t47007: f64, t1448: f64, t5591: f64, t48260: f64, t48262: f64, t13648: f64, t13716: f64, t22496: f64, t39773: f64, t39783: f64, t4139: f64, t47003: f64, t5532: f64, t5542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t73384 = 0.23392894490538584828e1_f64 * t48255;
    let t73388 = 192.0_f64 * t46999;
    let t73389 = 48.0_f64 * t47005;
    let t73390 = 96.0_f64 * t47007;
    let t73394 = t5591 * t1448;
    let t73398 = 0.46785788981077169656e1_f64 * t48260;
    let t73399 = 0.11696447245269292414e1_f64 * t48262;
    let t73400 = -12.0_f64 * t13648 * t22496 * t4139 + 6.0_f64 * t13716 * t4139 * t5532 - 12.0_f64 * t4139 * t5542 * t73394 + t39773 - t39783 + t47003 - t73384 + t73388 + t73389 + t73390 + t73398 - t73399;
    (t73384, t73388, t73389, t73390, t73398, t73399, t73400)
}
