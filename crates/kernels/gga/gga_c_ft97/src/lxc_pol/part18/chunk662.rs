//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 662/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk662<F: Float>(t11032: F, t11418: F, t348: F, t1537: F, t3108: F, t7733: F, t947: F, t3196: F, t8392: F, t1647: F, t3182: F, t1909: F, t3190: F, t1559: F, t920: F) -> (F, F, F, F, F, F, F, F) {
    let t11419 = t11032 + t11418;
    let t11420 = t348 * t11419;
    let t11424 = t1537 * t3108;
    let t11427 = t7733 * t947;
    let t11430 = 4.0 / 81.0 * t8392 * t3196;
    let t11431 = t3182 * t1647;
    let t11432 = t1909 * t11431;
    let t11436 = 4.0 / 27.0 * t8392 * t3190;
    let t11437 = t920 * t1559;
    (t11420, t11424, t11427, t11430, t11431, t11432, t11436, t11437)
}
