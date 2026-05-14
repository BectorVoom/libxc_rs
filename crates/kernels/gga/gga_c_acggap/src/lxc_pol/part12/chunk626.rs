//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 626/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk626<F: Float>(t339: F, t7329: F, t2028: F, t568: F, t1152: F, t1024: F, t56: F) -> (F, F, F, F, F) {
    let t7330 = t7329 * t339;
    let t7331 = 7.0 / 72.0 * t7330;
    let t7332 = t568 * t2028;
    let t7333 = t7332 * t1152;
    let t7335 = t56 * t1024;
    (t7330, t7331, t7332, t7333, t7335)
}
