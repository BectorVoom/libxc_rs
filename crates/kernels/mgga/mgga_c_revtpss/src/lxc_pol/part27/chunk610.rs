//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 610/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk610<F: Float>(t1248: F, t73: F, t1121: F, t471: F, t606: F, t3626: F, t126: F, t1263: F, t1122: F, t247: F, t1261: F, t1264: F, t3372: F) -> (F, F, F, F, F, F, F, F) {
    let t3627 = t1248 * t73;
    let t3628 = t471 * t1121;
    let t3629 = t3628 * t606;
    let t3630 = t3627 * t3629;
    let t3631 = t3626 * t3630;
    let t3634 = t126 * t1263;
    let t3635 = t3634 * t1122;
    let t3636 = t247 * t3635;
    let t3637 = t1261 * t3636;
    let t3639 = t1264 * t3372;
    (t3627, t3629, t3630, t3631, t3634, t3636, t3637, t3639)
}
