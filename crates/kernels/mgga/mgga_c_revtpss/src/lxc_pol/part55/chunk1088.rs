//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1088/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1088<F: Float>(t128196: F, t28168: F, t13648: F, t2014: F, t8714: F, t28056: F, t7359: F, t28696: F, t8634: F, t34242: F, t7315: F, t1353: F, t25082: F, t28286: F, t34297: F, t34270: F, t7239: F) -> (F, F, F, F, F, F, F) {
    let t128198 = 6.0 * t128196 * t28168;
    let t128200 = t2014 * t8714 * t13648;
    let t128204 = 2.0 * t7359 * t28056;
    let t128211 = 2.0 * t8634 * t28696;
    let t128219 = t2014 * t34242 * t7315;
    let t128223 = 6.0 * t25082 * t28286 * t34297 * t1353;
    let t128225 = 3.0 * t34270 * t7239;
    (t128198, t128200, t128204, t128211, t128219, t128223, t128225)
}
