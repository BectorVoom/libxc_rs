//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2183/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2183<F: Float>(t10073: F, t25403: F, t27198: F, t14939: F, t1955: F, t99270: F, t2471: F, t27202: F, t1559: F, t2828: F, t15003: F, t93194: F) -> (F, F, F, F, F, F) {
    let t99297 = t10073 * t27198 * t25403;
    let t99300 = t1955 * t14939;
    let t99303 = t1955 * t99270;
    let t99307 = t27202 * t2471;
    let t99309 = t1559 * t2828;
    let t99313 = t93194 * t15003;
    (t99297, t99300, t99303, t99307, t99309, t99313)
}
