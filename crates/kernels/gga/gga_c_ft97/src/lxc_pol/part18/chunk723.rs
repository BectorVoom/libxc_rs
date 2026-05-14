//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 723/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk723<F: Float>(t1009: F, t12411: F, t1010: F, t1015: F, t12368: F, t12371: F, t12374: F, t12381: F, t12385: F, t12392: F, t12397: F, t12402: F, t1683: F, t1698: F, t2001: F, t2059: F, t3348: F, t3350: F, t3356: F, t3381: F, t3387: F, t3392: F, t3393: F, t3406: F, t399: F) -> (F,) {
    let t12412 = t12411 * t1009;
    let t12417 = -4.0 * t2001 * t12368 - 2.0 * t2001 * t12371 - 4.0 * t12374 * t3356 - 0.38259118126557588605e1 * t3387 * t1683 + 0.38259118126557588605e1 * t1015 * t1683 + 8.0 * t2001 * t3393 * t12381 - 0.2416365355361531912e1 * t12385 * t399 - 0.1208182677680765956e1 * t3406 * t399 - 0.14597053826478655997e1 * t1015 * t1698 + 0.1208182677680765956e1 * t12392 * t399 + 0.29194107652957311994e1 * t3387 * t1698 - 6.0 * t3392 * t12397 * t2059 + 4.0 * t2001 * t12402 - 0.58388215305914623988e1 * t3350 * t1698 + 0.2416365355361531912e1 * t3381 * t399 + 0.29194107652957311994e1 * t1010 * t1698 - 0.2416365355361531912e1 * t12412 * t399 + 0.2416365355361531912e1 * t3348 * t399;
    (t12417,)
}
