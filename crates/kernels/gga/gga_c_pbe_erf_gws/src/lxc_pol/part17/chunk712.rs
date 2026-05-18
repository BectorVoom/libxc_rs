//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 712/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk712<F: Float>(t2409: F, t3067: F, t4164: F, t1125: F, t4023: F, t3132: F, t3139: F, t4028: F, t1140: F, t1184: F, t1150: F, t4039: F) -> (F, F, F, F, F, F) {
    let t4166 = t2409 * t3067 * t4164;
    let t4169 = t1125 * t4023;
    let t4171 = t3139 * t3132;
    let t4172 = t4028 * t4171;
    let t4174 = t1184 * t1140;
    let t4176 = t4039 * t1150;
    (t4166, t4169, t4171, t4172, t4174, t4176)
}
