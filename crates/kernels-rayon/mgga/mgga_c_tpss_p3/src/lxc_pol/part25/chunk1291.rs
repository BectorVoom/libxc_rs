//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1291/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1291(t198: f64, t206: f64, t6353: f64, t6337: f64, t768: f64, t63907: f64, t63913: f64, t63917: f64, t63928: f64, t63960: f64, t63966: f64, t63973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t66317 = t198 * t206 * t6353;
    let t66362 = t768 * t6337;
    let t66390 = 7.0_f64 / 144.0_f64 * t63907;
    let t66393 = 7.0_f64 / 144.0_f64 * t63913;
    let t66394 = 7.0_f64 / 288.0_f64 * t63917;
    let t66399 = 7.0_f64 / 576.0_f64 * t63928;
    let t66420 = 7.0_f64 / 144.0_f64 * t63960;
    let t66423 = 7.0_f64 / 36.0_f64 * t63966;
    let t66427 = 7.0_f64 / 576.0_f64 * t63973;
    (t66317, t66362, t66390, t66393, t66394, t66399, t66420, t66423, t66427)
}
