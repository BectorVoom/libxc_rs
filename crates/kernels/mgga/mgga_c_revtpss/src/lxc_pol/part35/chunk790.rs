//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 790/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk790<F: Float>(t14239: F, t5741: F, t6844: F, t72: F, t686: F, t4101: F, t6874: F, t545: F, t6888: F, t869: F, t689: F, t22005: F, t4003: F, t5744: F, t2782: F, t21981: F, t4086: F, t543: F) -> (F, F, F, F, F, F) {
    let t22329 = t14239 * t5741;
    let t22331 = t6844 * t72;
    let t22332 = t22331 * t686;
    let t22333 = t4101 * t22332;
    let t22335 = t6874 * t72;
    let t22336 = t22335 * t686;
    let t22337 = t4101 * t22336;
    let t22351 = t545 * t6888;
    let t22352 = t869 * t22351;
    let t22353 = t689 * t22352;
    let t22361 = t5744 * t22005 * t4003;
    let t22362 = t2782 * t22361;
    let t22365 = t4086 * t21981 * t543;
    (t22329, t22333, t22337, t22353, t22362, t22365)
}
