//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 918/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk918(t2336: f64, t4926: f64, t89: f64, t4918: f64, t9725: f64, t13723: f64, t13732: f64, t13740: f64, t14327: f64, t14329: f64, t14336: f64, t14341: f64, t14346: f64, t14347: f64, t18142: f64, t18145: f64, t18148: f64, t18153: f64, t18157: f64, t18162: f64, t18165: f64, t18168: f64, t9699: f64) -> (f64, f64, f64) {
    let t18171 = t89 * t2336 * t4926;
    let t18174 = t89 * t9725 * t4918;
    let t18176 = -t13723 - 2.0_f64 / 27.0_f64 * t13732 - t13740 - t14327 + t14329 - t18142 / 6.0_f64 - t18145 / 9.0_f64 + t18148 / 18.0_f64 - t14336 + t14341 - t14346 + t18153 / 3.0_f64 - t18157 / 18.0_f64 - t18162 + 2.0_f64 / 3.0_f64 * t18165 + t18168 / 54.0_f64 - t18171 / 27.0_f64 + t18174 / 81.0_f64 - t9699 - t14347;
    (t18171, t18174, t18176)
}
