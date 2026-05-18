//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 937/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk937<F: Float>(t10513: F, t657: F, t3544: F, t401: F, t3547: F, t3550: F, t3351: F, t5002: F, t422: F, t1714: F, t5063: F, t5061: F) -> (F, F, F, F, F, F, F, F) {
    let t10514 = t657 * t10513;
    let t10517 = t401 * t3544;
    let t10519 = t401 * t3547;
    let t10521 = t401 * t3550;
    let t10523 = t5002 * t3351;
    let t10524 = t10523 * t422;
    let t10525 = t1714 * t10524;
    let t10534 = t5063 * t3351;
    let t10535 = t10534 * t422;
    let t10536 = t5061 * t10535;
    (t10514, t10517, t10519, t10521, t10524, t10525, t10535, t10536)
}
