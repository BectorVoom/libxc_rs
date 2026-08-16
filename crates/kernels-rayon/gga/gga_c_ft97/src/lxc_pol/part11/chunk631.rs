//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 631/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk631(t637: f64, t639: f64, t8709: f64, t2253: f64, t2296: f64, t3626: f64, t70: f64, t170: f64, t180: f64, t645: f64, t8640: f64, t2265: f64, t631: f64, t8662: f64, t8665: f64, t8669: f64, t8672: f64, t8676: f64, t8678: f64, t8682: f64, t8686: f64) -> (f64, f64, f64) {
    let t8711 = t637 * t639 * t8709;
    let t8714 = t2253 * t2296;
    let t8715 = t3626 * t70;
    let t8718 = 20.0_f64 / 27.0_f64 * t170 * t8715 * t180;
    let t8719 = t8640 * t645;
    let t8721 = -t631 * t8662 / 3.0_f64 + t2265 * t8665 / 6.0_f64 - t2265 * t8669 - t2265 * t8672 + 4.0_f64 / 3.0_f64 * t8676 + 2.0_f64 / 3.0_f64 * t8678 + 3.0_f64 * t2265 * t8682 + 2.0_f64 * t2265 * t8686 + t631 * t8711 / 2.0_f64 - t8714 + t8718 + 5.0_f64 / 3.0_f64 * t8719;
    (t8711, t8715, t8721)
}
