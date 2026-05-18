//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 709/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk709<F: Float>(t10325: F, t184: F, t2790: F, t2796: F, t3345: F, t597: F, t1802: F, t3454: F, t5175: F, t3391: F, t582: F, t211: F) -> (F, F, F, F, F, F, F) {
    let t10326 = t10325 * t184;
    let t10329 = t2790 * t2796;
    let t10365 = t597 * t3345;
    let t10378 = t1802 * t3454;
    let t10383 = t5175 * t3454;
    let t10415 = t582 * t3391;
    let t10416 = t211 * t10415;
    (t10326, t10329, t10365, t10378, t10383, t10415, t10416)
}
