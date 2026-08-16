//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1247/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1247(t2022: f64, t9898: f64, t26028: f64, t9807: f64, t9812: f64, t2482: f64, t7262: f64, t814: f64, t9821: f64, t9958: f64, t820: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94413 = t2022 * t9898;
    let t94418 = t26028 * t9807;
    let t94420 = t26028 * t9812;
    let t94423 = t2482 * t7262 * t814;
    let t94424 = t94423 * t9821;
    let t94426 = t26028 * t9958;
    let t94429 = t820 * t7262 * t844;
    (t94413, t94418, t94420, t94424, t94426, t94429)
}
