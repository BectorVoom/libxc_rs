//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 646/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk646<F: Float>(t681: F, t6909: F, t89: F, t1901: F, t24758: F, t24815: F, t28375: F, t28379: F, t28382: F, t28384: F, t28388: F, t28392: F, t28395: F, t28398: F, t28401: F, t28405: F, t28408: F, t446: F) -> F {
    let t28411 = t89 * t681 * t6909;
    let t28413 = t24758 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t28375 - t1901 * t28379 / F::cast_from(9.0_f64) - t28382 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t28384 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t28388 - t446 * t28392 / F::cast_from(3.0_f64) - t24815 + t1901 * t28395 / F::cast_from(9.0_f64) + t1901 * t28398 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t28401 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t28405 - t28408 / F::cast_from(27.0_f64) - t28411 / F::cast_from(9.0_f64);
    t28413
}
