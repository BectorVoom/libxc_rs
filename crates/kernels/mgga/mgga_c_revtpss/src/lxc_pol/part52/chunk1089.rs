//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1089/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1089<F: Float>(t28056: F, t7359: F, t28696: F, t8634: F, t2014: F, t34242: F, t7315: F, t1353: F, t25082: F, t28286: F, t34297: F, t34270: F, t7239: F, t32737: F, t34495: F, t125939: F, t28196: F) -> (F, F, F, F, F, F, F) {
    let t128204 = 2.0 * t7359 * t28056;
    let t128211 = 2.0 * t8634 * t28696;
    let t128219 = t2014 * t34242 * t7315;
    let t128223 = 6.0 * t25082 * t28286 * t34297 * t1353;
    let t128225 = 3.0 * t34270 * t7239;
    let t128228 = 3.0 * t25082 * t34495 * t32737;
    let t128231 = 2.0 * t28196 * t28286 * t125939;
    (t128204, t128211, t128219, t128223, t128225, t128228, t128231)
}
