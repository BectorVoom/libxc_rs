//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1016/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1016(t19668: f64, t19729: f64, t845: f64, t91: f64, t14708: f64, t14711: f64, t14715: f64, t14718: f64, t15081: f64, t15082: f64, t15083: f64, t15089: f64, t15096: f64, t18999: f64) -> (f64, f64) {
    let t19730 = t19668 + t19729;
    let t19732 = t91 * t845 * t19730;
    let t19737 = -t15081 - t15082 + t15083 - t15089 - t15096 - t14708 + t19732 / 6.0_f64 - t14711 - 8.0_f64 / 81.0_f64 * t14715 - 4.0_f64 / 27.0_f64 * t14718 - 4.0_f64 / 9.0_f64 * t18999;
    (t19732, t19737)
}
