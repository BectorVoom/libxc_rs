//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1259/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1259<F: Float>(t10309: F, t1470: F, t1513: F, t94975: F, t530: F, t7933: F, t29411: F, t60224: F, t7565: F, t1479: F, t2282: F, t11239: F, t1811: F) -> (F, F, F, F, F, F, F) {
    let t101252 = t10309 * t1470;
    let t101451 = t94975 * t1513;
    let t101473 = t530 * t7933;
    let t104203 = t10309 * t29411;
    let t104208 = t60224 * t7565;
    let t104379 = t1479 * t2282;
    let t104527 = t1811 * t11239;
    (t101252, t101451, t101473, t104203, t104208, t104379, t104527)
}
