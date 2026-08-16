//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1159/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1159<F: Float>(t10301: F, t26178: F, t6960: F, t25114: F, t26179: F, t25102: F, t7349: F, t25120: F, t2247: F, t239: F, t38: F, t25163: F, t7348: F) -> (F, F, F, F, F, F) {
    let t95283 = t10301 * t26178;
    let t95284 = t95283 * t6960;
    let t95286 = t26179 * t25114;
    let t95288 = t25102 * t7349;
    let t95290 = t25120 * t7349;
    let t95293 = t2247 * t38 * t239;
    let t95294 = t95293 * t6960;
    let t95296 = t7348 * t25163;
    (t95284, t95286, t95288, t95290, t95294, t95296)
}
