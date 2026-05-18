//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 711/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk711<F: Float>(t1882: F, t6649: F, t6641: F, t379: F, t569: F, t6725: F, t1901: F, t23532: F, t27239: F, t27242: F, t27246: F, t27249: F, t27253: F, t27257: F, t27260: F, t27265: F, t27269: F, t27273: F, t3281: F, t446: F) -> F {
    let t27276 = t1882 * t6649;
    let t27278 = t1882 * t6641;
    let t27281 = t569 * t6725 * t379;
    let t27285 = -F::new(2.0) / F::new(9.0) * t1901 * t27239 - F::new(2.0) / F::new(9.0) * t1901 * t27242 + t1901 * t27246 / F::new(9.0) + t1901 * t27249 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t27253 - t1901 * t27257 / F::new(9.0) - t446 * t27260 / F::new(3.0) + t446 * t27265 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t27269 - F::new(2.0) / F::new(9.0) * t3281 * t27273 + t27276 / F::new(27.0) - t27278 / F::new(9.0) - t446 * t27281 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t23532;
    t27285
}
