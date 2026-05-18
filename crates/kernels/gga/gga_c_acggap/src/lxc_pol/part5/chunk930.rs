//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 930/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk930<F: Float>(t1195: F, t3670: F, t1036: F, t1037: F, t3266: F, t386: F, t1098: F, t3237: F, t1092: F, t1086: F, t941: F, t980: F) -> (F, F, F, F, F, F) {
    let t14309 = t3670 * t1195;
    let t14313 = t1036 * t386 * t3266 * t1037;
    let t14339 = t3237 * t1098;
    let t14341 = t3237 * t1092;
    let t14343 = t3237 * t1086;
    let t14345 = t980 * t941;
    (t14309, t14313, t14339, t14341, t14343, t14345)
}
