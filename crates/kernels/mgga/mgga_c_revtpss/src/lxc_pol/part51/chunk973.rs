//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 973/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk973<F: Float>(t121107: F, t5665: F, t121110: F, t1444: F, t1868: F, t120956: F, t1414: F, t828: F, t121090: F, t27888: F, t121093: F, t121019: F, t32284: F, t5700: F, t121018: F, t1399: F, t33962: F) -> (F, F, F, F, F, F, F, F) {
    let t125582 = t121107 * t5665;
    let t125584 = t121110 * t5665;
    let t125587 = t1868 * t1444;
    let t125590 = t120956 * t1414 * t828 * t125587;
    let t125594 = t121090 * t27888;
    let t125596 = t121093 * t27888;
    let t125599 = t32284 * t121019 * t5700;
    let t125603 = t121018 * t121019 * t33962 * t1399;
    (t125582, t125584, t125587, t125590, t125594, t125596, t125599, t125603)
}
