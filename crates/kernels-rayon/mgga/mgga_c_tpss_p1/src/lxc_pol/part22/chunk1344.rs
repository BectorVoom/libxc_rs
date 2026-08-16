//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1344/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1344(t63907: f64, t63913: f64, t63917: f64, t63899: f64, t63901: f64, t63903: f64, t63905: f64, t63909: f64, t63911: f64, t63921: f64, t63923: f64, t63925: f64) -> f64 {
    let t66390 = 7.0_f64 / 144.0_f64 * t63907;
    let t66393 = 7.0_f64 / 144.0_f64 * t63913;
    let t66394 = 7.0_f64 / 288.0_f64 * t63917;
    let t66398 = -t63899 / 384.0_f64 - t63901 / 768.0_f64 + t63903 / 96.0_f64 + t63905 / 192.0_f64 - t66390 + t63909 / 192.0_f64 + t63911 / 96.0_f64 - t66393 - t66394 - t63921 / 128.0_f64 + t63923 / 128.0_f64 - t63925 / 768.0_f64;
    t66398
}
