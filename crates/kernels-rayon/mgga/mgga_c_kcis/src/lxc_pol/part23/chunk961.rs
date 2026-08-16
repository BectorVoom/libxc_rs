//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 961/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk961(t16609: f64, t584: f64, t583: f64, t1546: f64, t17484: f64, t17488: f64, t17491: f64, t17494: f64, t17497: f64, t17499: f64, t17502: f64, t17506: f64, t17510: f64, t17512: f64, t17515: f64, t17518: f64, t17521: f64, t17693: f64, t17695: f64, t17698: f64, t17700: f64) -> (f64, f64, f64) {
    let t17702 = t584 * t16609;
    let t17703 = t583 * t17702;
    let t17704 = t1546 * t17703;
    let t17706 = t17484 / 256.0_f64 - t17488 / 16.0_f64 + t17491 / 12.0_f64 - t17494 / 9.0_f64 + t17497 / 6.0_f64 + t17499 / 18.0_f64 - t17502 / 48.0_f64 - t17506 / 18.0_f64 - 3.0_f64 / 8.0_f64 * t17510 - t17512 / 192.0_f64 + t17515 / 128.0_f64 - t17518 / 128.0_f64 + t17521 / 27.0_f64 + t17693 / 16.0_f64 + t17695 / 256.0_f64 + t17698 / 36.0_f64 - t17700 / 6.0_f64 + t17704 / 256.0_f64;
    (t17703, t17704, t17706)
}
