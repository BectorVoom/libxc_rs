//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1052/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1052<F: Float>(t14022: F, t885: F, t2149: F, t854: F, t6238: F, t899: F, t922: F, t2268: F, t2250: F, t3969: F, t933: F, t2191: F, t3065: F, t2159: F, t14028: F, t2308: F) -> (F, F, F, F, F, F, F, F) {
    let t51291 = t14022 * t885;
    let t51292 = t51291 * t2149;
    let t51293 = t854 * t51292;
    let t51301 = t899 * t6238 * t922;
    let t51302 = t51301 * t2268;
    let t51306 = t2250 * t3969 * t933;
    let t51309 = t3065 * t2191;
    let t51312 = t3065 * t2159;
    let t51315 = t14028 * t2308;
    (t51291, t51292, t51293, t51302, t51306, t51309, t51312, t51315)
}
