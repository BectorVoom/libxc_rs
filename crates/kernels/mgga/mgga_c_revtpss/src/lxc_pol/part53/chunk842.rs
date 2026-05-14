//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 842/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk842<F: Float>(t2007: F, t4292: F, t670: F, t7883: F, t1843: F, t7002: F, t651: F, t2322: F, t7742: F, t4254: F, t1310: F, t7741: F, t22496: F, t8717: F, t25082: F, t1469: F, t25129: F, t25132: F, t25137: F, t4181: F, t4186: F, t6968: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28050 = t2007 * t4292;
    let t28053 = t7883 * t670;
    let t28056 = t1843 * t7002;
    let t28058 = 2.0 * t651 * t28056;
    let t28060 = 2.0 * t2322 * t7742;
    let t28062 = 2.0 * t4254 * t7742;
    let t28063 = t1310 * t7741;
    let t28065 = 2.0 * t651 * t28063;
    let t28067 = t8717 * t22496;
    let t28069 = 3.0 * t25082 * t28067;
    let t28076 = -20.0 / 9.0 * t25129 * t1469 + 5.0 / 18.0 * t25132 * t4181 + 5.0 / 6.0 * t6968 * t4186 - t25137;
    (t28050, t28053, t28056, t28058, t28060, t28062, t28063, t28065, t28067, t28069, t28076)
}
