//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1228/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1228(t19926: f64, t7748: f64, t19934: f64, t19931: f64, t92447: f64, t5048: f64, t95351: f64, t5073: f64, t95381: f64, t1200: f64, t18463: f64, t100001: f64, t100003: f64, t100005: f64, t100007: f64, t100009: f64, t99984: f64, t99986: f64, t99988: f64, t99990: f64, t99992: f64, t99994: f64, t99997: f64, t99999: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100011 = t7748 * t19926;
    let t100013 = t7748 * t19934;
    let t100015 = t92447 * t19931;
    let t100017 = t95351 * t5048;
    let t100019 = t95381 * t5073;
    let t100021 = t18463 * t1200;
    let t100023 = t99984 / 16.0_f64 - t99986 / 8.0_f64 + t99988 / 128.0_f64 + t99990 / 3.0_f64 + t99992 / 9.0_f64 - t99994 / 32.0_f64 - t99997 / 16.0_f64 - t99999 / 36.0_f64 - t100001 / 6.0_f64 - t100003 / 96.0_f64 + t100005 / 36.0_f64 + t100007 / 12.0_f64 + t100009 / 48.0_f64 - t100011 / 12.0_f64 + t100013 / 72.0_f64 - 3.0_f64 / 8.0_f64 * t100015 + t100017 / 4.0_f64 + t100019 / 48.0_f64 - t100021 / 96.0_f64;
    (t100011, t100013, t100015, t100017, t100019, t100021, t100023)
}
