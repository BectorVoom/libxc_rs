//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 679/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk679<F: Float>(t3088: F, t3323: F, t1167: F, t2053: F, t1172: F, t319: F, t274: F, t331: F) -> (F, F, F, F) {
    let t3324 = t3088 + t3323;
    let t3327 = t1167 * t2053;
    let t3946 = t1172 * t319;
    let t3950 = t274 * t331;
    (t3324, t3327, t3946, t3950)
}
