//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1251/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1251(t33: f64, t4806: f64, t1398: f64, t1497: f64, t4802: f64, t1600: f64, t6323: f64, t1812: f64, t21255: f64, t18737: f64, t18746: f64, t19693: f64, t19706: f64, t19718: f64, t21274: f64, t21276: f64, t21278: f64, t21280: f64, t21282: f64, t21284: f64, t21286: f64, t21288: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21510 = t33 * t4806;
    let t21513 = t1497 * t1398;
    let t21516 = t33 * t4802;
    let t21576 = t1600 * t6323;
    let t21583 = t1812 * t21255;
    let t21608 = t18737 + 7.0_f64 / 36.0_f64 * t19693 + t21274 / 8.0_f64 - t21276 / 24.0_f64 + t21278 / 384.0_f64 + 7.0_f64 / 576.0_f64 * t19706 + t21280 / 96.0_f64 - t21282 / 768.0_f64 - t21284 / 768.0_f64 + t18746 + 7.0_f64 / 144.0_f64 * t19718 + 5.0_f64 / 192.0_f64 * t21286 - t21288 / 192.0_f64;
    (t21510, t21513, t21516, t21576, t21583, t21608)
}
