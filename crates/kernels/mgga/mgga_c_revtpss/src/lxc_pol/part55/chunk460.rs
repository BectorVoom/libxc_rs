//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 460/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk460<F: Float>(t3356: F, t1242: F, t472: F, t471: F, t1121: F, t414: F, t474: F, t479: F, t3089: F, t1285: F, t1264: F, t828: F, t1248: F, t73: F, t606: F, t126: F, t1263: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3579 = 0.19755555555555555556e-1 * t3356;
    let t3596 = 1.0 / t1242 / t472;
    let t3603 = t471 * t471;
    let t3617 = 1.0 / t414 / t1121;
    let t3623 = t474 * t479;
    let t3624 = t3623 * t3089;
    let t3625 = t1285 * t3624;
    let t3626 = t828 * t1264;
    let t3627 = t1248 * t73;
    let t3628 = t471 * t1121;
    let t3629 = t3628 * t606;
    let t3634 = t126 * t1263;
    (t3579, t3596, t3603, t3617, t3623, t3624, t3625, t3626, t3627, t3629, t3634)
}
