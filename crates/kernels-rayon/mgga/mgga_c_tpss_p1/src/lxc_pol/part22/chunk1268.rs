//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1268/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1268(t19693: f64, t19706: f64, t19718: f64, t17948: f64, t17962: f64, t17976: f64, t18737: f64, t18746: f64, t19698: f64, t19700: f64, t19704: f64, t19708: f64, t19710: f64, t19712: f64, t19716: f64, t19720: f64, t19722: f64) -> f64 {
    let t20434 = 7.0_f64 / 72.0_f64 * t19693;
    let t20438 = 7.0_f64 / 1152.0_f64 * t19706;
    let t20443 = 7.0_f64 / 288.0_f64 * t19718;
    let t20446 = t18737 + t17948 + t20434 + t19698 / 8.0_f64 - t19700 / 24.0_f64 + t19704 / 384.0_f64 + t20438 + t19708 / 192.0_f64 - t19710 / 768.0_f64 - t19712 / 768.0_f64 + t17962 + t18746 + t17976 + t19716 / 192.0_f64 + t20443 + 5.0_f64 / 192.0_f64 * t19720 - t19722 / 192.0_f64;
    t20446
}
