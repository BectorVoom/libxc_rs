//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2051/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2051<F: Float>(t1364: F, t26075: F, t786: F, t2482: F, t7262: F, t814: F, t9821: F, t820: F, t844: F, t3940: F, t596: F, t7269: F) -> (F, F, F, F, F, F) {
    let t94411 = t786 * t26075 * t1364;
    let t94423 = t2482 * t7262 * t814;
    let t94424 = t94423 * t9821;
    let t94429 = t820 * t7262 * t844;
    let t94430 = t94429 * t3940;
    let t94443 = t2482 * t7269 * t596;
    (t94411, t94423, t94424, t94429, t94430, t94443)
}
