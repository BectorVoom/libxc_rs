//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 536/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk536<F: Float>(t3223: F, t366: F, t362: F, t40: F, t611: F, t361: F, t351: F, t1054: F, t1058: F, t1014: F, t2857: F, t2251: F) -> (F, F, F, F, F, F) {
    let t3224 = t3223 * t366;
    let t3229 = F::cast_from(1.0_f64) / t40 / t362 / t611;
    let t3230 = t361 * t3229;
    let t3231 = t351 * t3230;
    let t3234 = t1054 * t1058;
    let t3236 = t1014 * t2857;
    let t3237 = t3236 * t2251;
    (t3224, t3229, t3230, t3231, t3234, t3237)
}
