//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 707/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk707<F: Float>(t29030: F, t296: F, t1901: F, t193: F, t24962: F, t25194: F, t25195: F, t29247: F, t29250: F, t29253: F, t29256: F, t29261: F, t29265: F, t29270: F, t29274: F, t29278: F, t446: F, t89: F) -> F {
    let t29281 = t296 * t29030;
    let t29284 = t24962 / F::new(9.0) + t446 * t29247 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t29250 + F::new(2.0) / F::new(3.0) * t446 * t29253 + t1901 * t29256 / F::new(9.0) + t1901 * t29261 / F::new(9.0) - t25194 + t89 * t193 * t29265 / F::new(3.0) + t25195 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t446 * t29270 - t446 * t29274 / F::new(9.0) + t446 * t29278 / F::new(3.0) - t446 * t29281 / F::new(3.0);
    t29284
}
