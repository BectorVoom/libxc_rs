//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 276/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk276<F: Float>(t2459: F, t676: F, t27: F, t89: F, t2335: F, t2339: F, t2342: F, t2352: F, t2357: F, t2364: F, t2368: F, t2376: F, t661: F, t754: F, t761: F) -> (F, F, F, F) {
    let t2460 = t676 * t2459;
    let t2462 = t89 * t27 * t2460;
    let t2464 = t2335 + t2339 + t2342 - t2352 / 27.0 + t2357 / 9.0 + t2364 / 9.0 - t2368 / 18.0 + t2376 / 3.0 - t2462 / 6.0;
    let t2465 = t661 * t2464;
    let t2469 = t754 * t761;
    (t2460, t2462, t2465, t2469)
}
