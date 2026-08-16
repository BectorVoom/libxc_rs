//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1365/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1365(t65570: f64, t65592: f64, t65600: f64, t65572: f64, t65574: f64, t65576: f64, t65578: f64, t65580: f64, t65582: f64, t65584: f64, t65586: f64, t65588: f64, t65597: f64) -> f64 {
    let t67150 = 7.0_f64 / 36.0_f64 * t65570;
    let t67160 = 7.0_f64 / 288.0_f64 * t65592;
    let t67162 = 7.0_f64 / 12.0_f64 * t65600;
    let t67163 = t67150 - t65572 / 24.0_f64 + t65574 / 192.0_f64 - t65576 / 384.0_f64 - t65578 / 768.0_f64 + t65580 / 128.0_f64 + t65582 / 96.0_f64 + t65584 / 192.0_f64 - t65586 / 96.0_f64 - 5.0_f64 / 192.0_f64 * t65588 - t67160 - t65597 / 2.0_f64 - t67162;
    t67163
}
