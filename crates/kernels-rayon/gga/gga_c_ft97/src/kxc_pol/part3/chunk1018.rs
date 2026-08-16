//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1018/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1018(t19752: f64, t856: f64, t91: f64, t4191: f64, t4226: f64, t10631: f64, t5337: f64, t19246: f64, t19249: f64, t19252: f64, t19255: f64, t19258: f64, t19261: f64, t19265: f64, t19269: f64) -> (f64, f64, f64, f64) {
    let t19754 = t91 * t19752 * t856;
    let t19757 = t91 * t4191 * t4226;
    let t19759 = t10631 * t5337;
    let t19761 = t91 * t19759 * t856;
    let t19769 = t19246 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t19249 - t19754 / 12.0_f64 - t19757 / 6.0_f64 + t19761 / 8.0_f64 + 2.0_f64 / 9.0_f64 * t19252 - 2.0_f64 / 27.0_f64 * t19255 - 10.0_f64 / 81.0_f64 * t19258 + 8.0_f64 / 27.0_f64 * t19261 + 2.0_f64 / 9.0_f64 * t19265 - 4.0_f64 / 9.0_f64 * t19269;
    (t19754, t19757, t19761, t19769)
}
