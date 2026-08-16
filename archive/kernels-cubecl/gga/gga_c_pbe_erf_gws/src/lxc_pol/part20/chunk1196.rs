//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1196/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1196<F: Float>(t15185: F, t15285: F, t15330: F, t15380: F, t1167: F, t14821: F, t14153: F, t3931: F, t3928: F, t4063: F, t360: F, t898: F) -> (F, F, F, F, F) {
    let t15382 = t15185 + t15285 + t15330 + t15380;
    let t15386 = t14821 * t1167;
    let t15389 = t14153 * t3931;
    let t15392 = t4063 * t3928;
    let t15636 = t898 * t360;
    (t15382, t15386, t15389, t15392, t15636)
}
