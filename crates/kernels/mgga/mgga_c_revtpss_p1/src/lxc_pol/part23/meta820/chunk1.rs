//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2670/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2670<F: Float>(t15707: F, t15769: F, t12013: F, t20029: F, t1063: F, t19671: F, t3172: F, t19697: F, t3173: F, t1041: F, t19799: F, t11262: F, t6301: F) -> (F, F, F, F, F, F) {
    let t65931 = t15707 * t15769;
    let t65960 = t12013 * t20029;
    let t65965 = t1063 * t3172 * t19671;
    let t66003 = t19697 * t3173;
    let t66017 = t1041 * t3172 * t19799;
    let t66022 = t1041 * t11262 * t6301;
    (t65931, t65960, t65965, t66003, t66017, t66022)
}
