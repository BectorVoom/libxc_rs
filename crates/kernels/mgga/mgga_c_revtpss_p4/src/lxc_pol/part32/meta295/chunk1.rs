//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1198/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1198<F: Float>(t10162: F, t2439: F, t1420: F, t2453: F, t3908: F, t1426: F, t786: F, t64: F, t843: F, t112: F, t2289: F, t666: F) -> (F, F, F, F, F, F) {
    let t10163 = t2439 * t10162;
    let t10165 = t2453 * t1420;
    let t10166 = t10165 * t3908;
    let t10174 = t1420 * t1426;
    let t10175 = t786 * t10174;
    let t10199 = t64 * t843;
    let t10201 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t10199 * t112;
    let t10202 = t2289 * t666;
    (t10163, t10166, t10175, t10199, t10201, t10202)
}
