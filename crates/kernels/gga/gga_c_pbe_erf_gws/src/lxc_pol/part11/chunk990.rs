//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 990/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk990<F: Float>(t2106: F, t3772: F, t1: F, t2057: F, t2062: F, t3701: F, t10024: F, t2096: F, t11387: F, t331: F, t4395: F, t3916: F, t6154: F) -> (F, F, F, F, F) {
    let t35109 = t3772 * t2106;
    let t35128 = t3701 * t2057 * t1 * t2062;
    let t35137 = t10024 * t2096;
    let t35187 = t11387 * t331;
    let t35188 = t4395 * t35187;
    let t35277 = t3916 * t6154;
    (t35109, t35128, t35137, t35188, t35277)
}
