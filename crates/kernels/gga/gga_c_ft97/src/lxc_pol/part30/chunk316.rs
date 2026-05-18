//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 316/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk316<F: Float>(t295: F, t312: F, t4239: F, t1901: F, t193: F, t2839: F, t2872: F, t3281: F, t4142: F, t4147: F, t4152: F, t4156: F, t4159: F, t4164: F, t4169: F, t4173: F, t4178: F, t4183: F, t4188: F, t446: F, t89: F) -> F {
    let t4241 = t295 * t4239 * t312;
    let t4245 = -F::new(2.0) / F::new(27.0) * t1901 * t4142 + t1901 * t4147 / F::new(9.0) + t1901 * t4152 / F::new(9.0) + t2872 / F::new(27.0) + t4156 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t3281 * t4159 + F::new(2.0) / F::new(3.0) * t446 * t4164 + t446 * t4169 / F::new(3.0) - t446 * t4173 / F::new(9.0) + t446 * t4178 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t4183 - t2839 / F::new(9.0) - t446 * t4188 / F::new(9.0) + t89 * t193 * t4241 / F::new(3.0);
    t4245
}
