//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 440/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk440<F: Float>(t2459: F, t676: F, t27: F, t89: F, t2335: F, t2339: F, t2342: F, t2352: F, t2357: F, t2364: F, t2368: F, t2376: F) -> (F, F, F) {
    let t2460 = t676 * t2459;
    let t2462 = t89 * t27 * t2460;
    let t2464 = t2335 + t2339 + t2342 - t2352 / F::new(27.0) + t2357 / F::new(9.0) + t2364 / F::new(9.0) - t2368 / F::new(18.0) + t2376 / F::new(3.0) - t2462 / F::new(6.0);
    (t2460, t2462, t2464)
}
