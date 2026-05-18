//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 970/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk970<F: Float>(t125: F, t1444: F, t246: F, t551: F, t32276: F, t239: F, t3999: F, t8589: F, t8583: F) -> (F, F, F, F, F, F) {
    let t32277 = t125 * t1444;
    let t32278 = t246 * t32277;
    let t32279 = t551 * t32278;
    let t32280 = t32276 * t32279;
    let t32282 = t3999 * t239;
    let t32283 = t8589 * t32282;
    let t32284 = t8583 * t32283;
    (t32278, t32279, t32280, t32282, t32283, t32284)
}
