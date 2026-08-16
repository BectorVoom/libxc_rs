//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 271/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk271<F: Float>(t403: F, t404: F, t1121: F, t606: F, t1012: F, t1204: F, t225: F, t480: F, t1209: F) -> (F, F, F, F, F, F, F) {
    let t1224 = F::cast_from(1.0_f64) / t404 / t403;
    let t1225 = t1224 * t1121;
    let t1226 = t1225 * t606;
    let t1227 = t1012 * t1226;
    let t1230 = t1204 * t225;
    let t1231 = t1230 * t480;
    let t1234 = t1209 * t225;
    (t1224, t1225, t1226, t1227, t1230, t1231, t1234)
}
