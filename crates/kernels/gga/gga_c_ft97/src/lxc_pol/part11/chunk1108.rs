//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1108/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1108<F: Float>(t10907: F, t2253: F, t2937: F, t326: F, t2939: F, t3628: F, t906: F, t10894: F, t10916: F, t2265: F, t231: F, t2917: F, t2918: F, t2923: F, t39370: F, t41448: F, t41468: F, t43188: F, t43190: F, t43192: F, t43194: F, t43195: F, t43200: F, t43202: F, t43223: F, t43241: F, t631: F, t684: F, t893: F, t898: F, t900: F) -> F {
    let t43247 = t2253 * t10907;
    let t43250 = F::cast_from(1.0_f64) / t2937 / t326;
    let t43251 = t2939 * t2939;
    let t43268 = t3628 * t906;
    let t43270 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t2923 * t684 * t10894 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t43188 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t43190 - F::cast_from(10.0_f64) * t43192 + F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t631 * t43194 * t43195 * t41448 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t43200 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t43202 + t631 * t898 * t900 * (t43223 + t43241) / F::cast_from(2.0_f64) - F::cast_from(16.0_f64) * t43247 - F::cast_from(30.0_f64) * t631 * t898 * t43250 * t43251 + t631 * t231 * t893 * t39370 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) * t631 * t2917 * t10916 * t41448 + t631 * t2917 * t2918 * t41468 / F::cast_from(6.0_f64) - F::cast_from(160.0_f64) / F::cast_from(27.0_f64) * t43268;
    t43270
}
