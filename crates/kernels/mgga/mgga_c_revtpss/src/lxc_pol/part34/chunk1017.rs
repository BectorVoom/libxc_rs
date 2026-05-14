//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1017/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1017<F: Float>(t26012: F, t1941: F, t550: F, t1389: F, t25240: F, t3964: F, t7262: F, t820: F, t843: F, t241: F) -> (F, F, F, F, F) {
    let t26013 = 0.15244095330869239812e-3 * t26012;
    let t26017 = t1941 * t550;
    let t26021 = t3964 * t25240 * t1389;
    let t26022 = 0.90357964994909313586e-5 * t26021;
    let t26024 = t820 * t7262 * t843;
    let t26028 = t820 * t7262 * t241;
    (t26013, t26017, t26022, t26024, t26028)
}
