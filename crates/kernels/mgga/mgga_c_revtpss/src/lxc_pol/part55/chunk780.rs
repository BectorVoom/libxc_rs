//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 780/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk780<F: Float>(t4075: F, t7282: F, t1955: F, t1385: F, t2022: F, t1426: F, t545: F, t10073: F, t2453: F, t7283: F, t136: F, t2029: F, t2457: F, t25920: F, t7063: F, t7286: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25929 = t7282 * t4075;
    let t25930 = t1955 * t25929;
    let t25931 = t1385 * t2022;
    let t25937 = t1426 * t545;
    let t25938 = t25937 * t2022;
    let t25939 = t7282 * t25938;
    let t25941 = 0.24093411633903331839e-3 * t10073 * t25939;
    let t25944 = t2453 * t7283;
    let t25945 = t2029 * t136;
    let t25946 = t25945 * t2457;
    let t25948 = 0.17135234354032049604e-2 * t25944 * t25946;
    let t25949 = t25920 * t1426;
    let t25950 = t7063 * t25949;
    let t25951 = t25950 * t7286;
    (t25930, t25931, t25937, t25938, t25941, t25944, t25946, t25948, t25949, t25950, t25951)
}
