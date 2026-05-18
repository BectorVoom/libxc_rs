//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 911/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk911<F: Float>(t2970: F, t6173: F, t3014: F, t6205: F, t2926: F, t6141: F, t342: F, t6343: F, t6234: F, t993: F, t225: F, t3011: F) -> (F, F, F, F, F, F, F) {
    let t19275 = t6173 * t2970;
    let t19303 = t6205 * t3014;
    let t19330 = t6141 * t2926;
    let t19351 = t342 * t6343;
    let t19462 = t6234 * t993;
    let t19463 = t19462 * t225;
    let t19467 = t3011 * t6205;
    (t19275, t19303, t19330, t19351, t19462, t19463, t19467)
}
