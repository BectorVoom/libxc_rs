//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 790/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk790<F: Float>(t8589: F, t938: F, t829: F, t830: F, t2373: F, t3083: F, t3205: F, t858: F, t1118: F, t810: F, t353: F, t2501: F, t814: F, t2367: F, t3052: F, t1114: F, t4384: F) -> (F, F, F, F, F, F, F, F) {
    let t8590 = t8589 * t938;
    let t8592 = t829 * t830 * t8590;
    let t8598 = 7.0 / 72.0 * t3083 * t2373;
    let t8599 = t3205 * t858;
    let t8600 = t1118 * t810;
    let t8601 = t353 * t8600;
    let t8602 = t8599 * t8601;
    let t8611 = t829 * t830 * t2501 * t814;
    let t8622 = 7.0 / 72.0 * t2367 * t3052;
    let t8629 = t1114 * t4384;
    (t8590, t8592, t8598, t8599, t8602, t8611, t8622, t8629)
}
