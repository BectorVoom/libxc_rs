//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 933/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk933<F: Float>(t86: F, t112: F, t113: F, t1927: F, t1934: F, t38381: F, t39358: F, t39370: F, t5: F, t502: F, t505: F, t8598: F, t8608: F) -> F {
    let t87 = F::cast_from(10000000.0_f64) <= t86;
    let t39375 = piecewise3::<F>(t87, F::new(0.0), t5 * (t38381 + t39358) * t113 / F::new(4.0) + t5 * t8598 * t505 + F::new(3.0) / F::new(2.0) * t5 * t1927 * t1934 + t5 * t502 * t8608 + t5 * t112 * t39370 / F::new(4.0));
    t39375
}
