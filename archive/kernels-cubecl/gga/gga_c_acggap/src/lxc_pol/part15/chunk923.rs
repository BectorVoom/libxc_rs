//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 923/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk923<F: Float>(t2092: F, t7780: F, t154: F, t2096: F, t31035: F, t3036: F, t597: F, t137: F, t3037: F, t1089: F, t1095: F, t2113: F) -> (F, F, F, F, F, F) {
    let t31505 = t7780 * t2092;
    let t31508 = t31035 * t154 * t2096;
    let t31520 = t3036 * t597;
    let t31521 = t137 * t3037;
    let t31524 = t31520 * t1089 * t1095 * t31521;
    let t31526 = t7780 * t2113;
    (t31505, t31508, t31520, t31521, t31524, t31526)
}
