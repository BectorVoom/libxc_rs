//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1416/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1416(t23379: f64, t449: f64, t446: f64, t2132: f64, t5407: f64, t6290: f64, t908: f64, t1881: f64, t5414: f64, t13000: f64, t13096: f64, t18385: f64, t20856: f64, t20859: f64, t20861: f64, t20863: f64, t20866: f64, t20870: f64, t9267: f64, t9270: f64, t9278: f64, t9281: f64) -> f64 {
    let t23380 = t449 * t23379;
    let t23381 = t446 * t23380;
    let t23383 = t5407 * t2132;
    let t23384 = t446 * t23383;
    let t23386 = t6290 * t908;
    let t23387 = t1881 * t5414;
    let t23389 = -t20856 / 8.0_f64 - t20859 / 16.0_f64 + t13096 + t20861 / 8.0_f64 + t20863 / 16.0_f64 - t20866 / 16.0_f64 + 2.0_f64 * t18385 - t20870 / 16.0_f64 - t9278 + t9267 - t23381 / 16.0_f64 + t9281 - t23384 / 8.0_f64 + t23386 + t23387 / 8.0_f64 - t9270 + t13000;
    t23389
}
