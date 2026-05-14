//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 80/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk80<F: Float>(t27: F, t13: F, t14: F, t1: F, t3: F, t6: F, t78: F) -> (F, F, F, F, F) {
    let t338 = t27 * t27;
    let t339 = 1.0 / t338;
    let t340 = t13 * t339;
    let t341 = 1.0 / t14;
    let t342 = t341 * t1;
    let t343 = t3 * t6;
    let t344 = t343 * t78;
    let t345 = t342 * t344;
    (t340, t341, t343, t344, t345)
}
