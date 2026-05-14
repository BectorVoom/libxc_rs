//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1054/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1054<F: Float>(t2118: F, t2132: F, t822: F, t2263: F, t331: F, t56: F, t863: F, t14092: F, t6706: F, t14028: F, t2339: F, t14022: F, t885: F, t2149: F, t854: F, t6238: F, t899: F, t922: F) -> (F, F, F, F, F, F, F, F, F) {
    let t51266 = t2118 * t2132;
    let t51267 = t822 * t51266;
    let t51274 = t863 * t2263 * t331 * t56;
    let t51282 = t14092 * t6706;
    let t51285 = t14028 * t2339;
    let t51291 = t14022 * t885;
    let t51292 = t51291 * t2149;
    let t51293 = t854 * t51292;
    let t51301 = t899 * t6238 * t922;
    (t51266, t51267, t51274, t51282, t51285, t51291, t51292, t51293, t51301)
}
