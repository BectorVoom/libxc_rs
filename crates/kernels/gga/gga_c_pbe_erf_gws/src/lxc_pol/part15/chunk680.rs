//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 680/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk680<F: Float>(t1172: F, t320: F, t1198: F, t2053: F, t1105: F, t3944: F, t1123: F, t3950: F, t850: F, t833: F, t2409: F, t3050: F, t3959: F, t1146: F, t1173: F, t3045: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4062 = t1172 * t320;
    let t4063 = t1198 * t2053;
    let t4123 = t3944 * t1105;
    let t4127 = t850 * t1123 * t3950;
    let t4128 = t4127 * t833;
    let t4130 = t2409 * t3050;
    let t4131 = t3959 * t4130;
    let t4133 = t1173 * t1146;
    let t4135 = t2409 * t3045;
    (t4062, t4063, t4123, t4127, t4128, t4130, t4131, t4133, t4135)
}
