//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1108/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1108(t10907: f64, t2253: f64, t2937: f64, t326: f64, t2939: f64, t3628: f64, t906: f64, t10894: f64, t10916: f64, t2265: f64, t231: f64, t2917: f64, t2918: f64, t2923: f64, t39370: f64, t41448: f64, t41468: f64, t43188: f64, t43190: f64, t43192: f64, t43194: f64, t43195: f64, t43200: f64, t43202: f64, t43223: f64, t43241: f64, t631: f64, t684: f64, t893: f64, t898: f64, t900: f64) -> f64 {
    let t43247 = t2253 * t10907;
    let t43250 = 1.0_f64 / t2937 / t326;
    let t43251 = t2939 * t2939;
    let t43268 = t3628 * t906;
    let t43270 = -4.0_f64 / 3.0_f64 * t2265 * t2923 * t684 * t10894 - 4.0_f64 / 3.0_f64 * t43188 + 10.0_f64 / 3.0_f64 * t43190 - 10.0_f64 * t43192 + 14.0_f64 / 81.0_f64 * t631 * t43194 * t43195 * t41448 + 10.0_f64 / 9.0_f64 * t43200 + 10.0_f64 / 27.0_f64 * t43202 + t631 * t898 * t900 * (t43223 + t43241) / 2.0_f64 - 16.0_f64 * t43247 - 30.0_f64 * t631 * t898 * t43250 * t43251 + t631 * t231 * t893 * t39370 / 6.0_f64 + 2.0_f64 * t631 * t2917 * t10916 * t41448 + t631 * t2917 * t2918 * t41468 / 6.0_f64 - 160.0_f64 / 27.0_f64 * t43268;
    t43270
}
