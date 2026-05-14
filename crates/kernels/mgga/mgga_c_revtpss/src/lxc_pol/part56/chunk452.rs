//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 452/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk452<F: Float>(t1248: F, t73: F, t1121: F, t471: F, t606: F, t126: F, t1263: F, t1122: F, t247: F, t1261: F, t1230: F, t1260: F, t371: F, t482: F, t676: F, t481: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3627 = t1248 * t73;
    let t3628 = t471 * t1121;
    let t3629 = t3628 * t606;
    let t3634 = t126 * t1263;
    let t3635 = t3634 * t1122;
    let t3636 = t247 * t3635;
    let t3637 = t1261 * t3636;
    let t3647 = t1230 * t1260;
    let t3655 = t371 * t676 * t482;
    let t3657 = 0.47637797908966374413e-4 * t481 * t3655;
    (t3627, t3629, t3634, t3635, t3636, t3637, t3647, t3655, t3657)
}
