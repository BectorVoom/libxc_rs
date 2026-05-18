//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 714/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk714<F: Float>(t1212: F, t6386: F, t840: F, t871: F, t681: F, t7093: F, t89: F, t25298: F, t25312: F, t25315: F, t25317: F, t25366: F, t29378: F, t29383: F, t29385: F, t29387: F, t29389: F, t29392: F, t29396: F, t446: F) -> (F, F) {
    let t29399 = t6386 * t1212;
    let t29401 = t840 * t871 * t29399;
    let t29405 = t89 * t681 * t7093;
    let t29407 = t25298 / F::new(9.0) + t25312 / F::new(27.0) - t446 * t29378 / F::new(3.0) - t25315 / F::new(9.0) + t25317 / F::new(9.0) - t29383 / F::new(9.0) + t29385 / F::new(9.0) + t29387 / F::new(9.0) - t446 * t29389 / F::new(3.0) + t29392 / F::new(9.0) + t25366 / F::new(9.0) + t446 * t29396 / F::new(3.0) + t446 * t29401 / F::new(3.0) - t29405 / F::new(9.0);
    (t29399, t29407)
}
