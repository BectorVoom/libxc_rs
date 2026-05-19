//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1172/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1172<F: Float>(t299: F, t43297: F, t44795: F, t10188: F, t10944: F, t13: F, t39375: F, t41401: F, t43034: F, t8613: F, t9479: F) -> F {
    let t300 = F::cast_from(10000000.0_f64) <= t299;
    let t44797 = piecewise3::<F>(t300, F::new(0.0), t43297 + t44795);
    let tv4rho40 = F::new(4.0) * t8613 + F::new(4.0) * t9479 + F::new(4.0) * t10188 + F::new(4.0) * t10944 + t13 * (t39375 + t41401 + t43034 + t44797);
    tv4rho40
}
