//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 943/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk943(t13812: f64, t18153: f64, t18157: f64, t18162: f64, t18165: f64, t18168: f64, t18171: f64, t18174: f64, t18372: f64, t18375: f64, t9972: f64, t18557: f64, t18567: f64, t18575: f64) -> f64 {
    let t18585 = t18372 / 8.0_f64 - t18375 / 6.0_f64 + 2.0_f64 / 3.0_f64 * t18153 - t18157 / 9.0_f64 - 2.0_f64 * t18162 + 4.0_f64 / 3.0_f64 * t18165 + t18168 / 27.0_f64 - 2.0_f64 / 27.0_f64 * t18171 + 2.0_f64 / 81.0_f64 * t18174 - t9972 - t13812;
    let t18587 = t18557 + t18567 + t18575 + t18585;
    t18587
}
