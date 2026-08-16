//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1346/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1346(t63957: f64, t63960: f64, t63964: f64, t63966: f64, t61063: f64, t61065: f64, t61073: f64, t62711: f64, t63951: f64, t63953: f64, t63955: f64, t63962: f64, t63968: f64) -> f64 {
    let t66418 = 35.0_f64 / 108.0_f64 * t63957;
    let t66420 = 7.0_f64 / 144.0_f64 * t63960;
    let t66422 = 119.0_f64 / 864.0_f64 * t63964;
    let t66423 = 7.0_f64 / 36.0_f64 * t63966;
    let t66425 = -35.0_f64 / 54.0_f64 * t61063 + 7.0_f64 / 72.0_f64 * t61065 - t63951 / 48.0_f64 + t63953 / 192.0_f64 + t63955 / 384.0_f64 - t66418 - 7.0_f64 / 24.0_f64 * t61073 + t66420 - t63962 / 192.0_f64 - t66422 - t62711 + t66423 - t63968 / 24.0_f64;
    t66425
}
