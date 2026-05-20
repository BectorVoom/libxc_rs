//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1302/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1302<F: Float>(t1592: F, t999: F, t1045: F, t15691: F, t1066: F, t18946: F, t247: F, t11725: F, t6092: F, t1063: F, t3109: F, t6100: F) -> (F, F, F, F) {
    let t20038 = t1592 * t999;
    let t20039 = t1045 * t20038;
    let t20040 = t15691 * t20039;
    let t20046 = t247 * t1066 * t18946;
    let t20050 = t247 * t11725 * t6092;
    let t20051 = t1063 * t20050;
    let t20054 = t247 * t3109 * t6100;
    (t20040, t20046, t20051, t20054)
}
