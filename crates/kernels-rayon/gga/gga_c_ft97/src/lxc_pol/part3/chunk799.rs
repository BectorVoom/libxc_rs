//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 799/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk799(t15885: f64, t24: f64, t469: f64, t15917: f64, t1787: f64, t15752: f64, t3134: f64, t11668: f64, t11669: f64, t11684: f64, t11686: f64, t16370: f64, t16373: f64, t16375: f64, t16378: f64, t16381: f64, t16384: f64, t16387: f64, t16392: f64, t16396: f64, t16401: f64, t16404: f64, t16406: f64, t3139: f64, t462: f64, t8283: f64, t92: f64) -> f64 {
    let t16409 = t24 * t469 * t15885;
    let t16411 = t1787 * t15917;
    let t16414 = t3134 * t15752;
    let t16417 = t11668 - 8.0_f64 / 9.0_f64 * t11669 - 4.0_f64 / 27.0_f64 * t8283 + t462 * t16370 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t16373 - 2.0_f64 / 3.0_f64 * t462 * t16375 + t462 * t16378 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t462 * t16381 - 2.0_f64 / 9.0_f64 * t462 * t16384 + 4.0_f64 / 3.0_f64 * t3139 * t16387 + 2.0_f64 * t462 * t16392 - t462 * t16396 / 3.0_f64 - 6.0_f64 * t462 * t16401 - t11684 + t11686 + t16404 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t16406 - t92 * t16409 - 2.0_f64 / 3.0_f64 * t462 * t16411 - 2.0_f64 * t462 * t16414;
    t16417
}
