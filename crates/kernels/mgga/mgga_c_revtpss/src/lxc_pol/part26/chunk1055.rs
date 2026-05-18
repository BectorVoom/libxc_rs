//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1055/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1055<F: Float>(t545: F, t9656: F, t4075: F, t7282: F, t1955: F, t1398: F, t1444: F, t543: F, t1426: F, t2453: F, t7283: F, t25920: F) -> (F, F, F, F, F, F, F) {
    let t25924 = t9656 * t545;
    let t25929 = t7282 * t4075;
    let t25930 = t1955 * t25929;
    let t25933 = t1444 * t1398 * t543;
    let t25937 = t1426 * t545;
    let t25944 = t2453 * t7283;
    let t25949 = t25920 * t1426;
    (t25924, t25929, t25930, t25933, t25937, t25944, t25949)
}
