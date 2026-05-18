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
    let t43250 = F::new(1.0) / t2937 / t326;
    let t43251 = t2939 * t2939;
    let t43268 = t3628 * t906;
    let t43270 = -F::new(4.0) / F::new(3.0) * t2265 * t2923 * t684 * t10894 - F::new(4.0) / F::new(3.0) * t43188 + F::new(10.0) / F::new(3.0) * t43190 - F::new(10.0) * t43192 + F::new(14.0) / F::new(81.0) * t631 * t43194 * t43195 * t41448 + F::new(10.0) / F::new(9.0) * t43200 + F::new(10.0) / F::new(27.0) * t43202 + t631 * t898 * t900 * (t43223 + t43241) / F::new(2.0) - F::new(16.0) * t43247 - F::new(30.0) * t631 * t898 * t43250 * t43251 + t631 * t231 * t893 * t39370 / F::new(6.0) + F::new(2.0) * t631 * t2917 * t10916 * t41448 + t631 * t2917 * t2918 * t41468 / F::new(6.0) - F::new(160.0) / F::new(27.0) * t43268;
    t43270
}
