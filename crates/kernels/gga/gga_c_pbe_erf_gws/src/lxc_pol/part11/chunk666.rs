//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 666/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk666<F: Float>(t10068: F, t133: F, t10071: F, t10037: F, t525: F, t285: F, t3379: F, t545: F, t281: F, t2030: F, t985: F, t299: F, t169: F, t242: F, t3689: F, t700: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10168 = t133 * t10068;
    let t10170 = t133 * t10071;
    let t10186 = t525 * t10037;
    let t10207 = t3379 * t545 * t285;
    let t10208 = t281 * t10207;
    let t10222 = t2030 * t985;
    let t10229 = t299 * t3379;
    let t10231 = t169 * t10229 * t242;
    let t10239 = t169 * t3689 * t700;
    (t10168, t10170, t10186, t10207, t10208, t10222, t10229, t10231, t10239)
}
