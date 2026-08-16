//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 782/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk782<F: Float>(t1980: F, t2672: F, t1392: F, t2581: F, t1391: F, t2571: F, t2013: F, t2680: F, t2012: F, t2683: F) -> (F, F, F, F, F) {
    let t7403 = t1980 * t2672;
    let t7406 = t1392 * t2581;
    let t7407 = t1391 * t7406;
    let t7410 = t1392 * t2571;
    let t7411 = t1391 * t7410;
    let t7414 = t2013 * t2680;
    let t7416 = t2012 * t2683;
    (t7403, t7407, t7411, t7414, t7416)
}
