//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1182/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1182<F: Float>(t1398: F, t1444: F, t543: F, t25931: F, t1426: F, t545: F, t2022: F, t7282: F, t10073: F, t2453: F, t7283: F, t136: F, t2029: F) -> (F, F, F, F, F, F, F, F) {
    let t25933 = t1444 * t1398 * t543;
    let t25934 = t25931 * t25933;
    let t25937 = t1426 * t545;
    let t25938 = t25937 * t2022;
    let t25939 = t7282 * t25938;
    let t25941 = F::new(0.24093411633903331839e-3) * t10073 * t25939;
    let t25944 = t2453 * t7283;
    let t25945 = t2029 * t136;
    (t25933, t25934, t25937, t25938, t25939, t25941, t25944, t25945)
}
