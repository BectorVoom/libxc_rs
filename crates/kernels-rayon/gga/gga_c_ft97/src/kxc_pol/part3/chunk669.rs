//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 669/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk669(t255: f64, t9802: f64, t2347: f64, t761: f64, t731: f64, t8232: f64, t768: f64, t9735: f64, t9701: f64, t251: f64, t631: f64, t675: f64, t7242: f64, t898: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9803 = t9802 * t255;
    let t9808 = t761 * t2347;
    let t9822 = t8232 * t731;
    let t9824 = t8232 * t768;
    let t9863 = 4.0_f64 / 27.0_f64 * t9735;
    let t9867 = 4.0_f64 / 9.0_f64 * t9701;
    let t9890 = 1.0_f64 / t251 / t631 / t898 / t675 / t7242 / 4.0_f64;
    (t9803, t9808, t9822, t9824, t9863, t9867, t9890)
}
