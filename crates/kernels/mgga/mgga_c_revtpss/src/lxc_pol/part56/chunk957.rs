//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 957/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk957<F: Float>(t120962: F, t32284: F, t5705: F, t5696: F, t120952: F, t1885: F, t5661: F, t121107: F, t5665: F, t121110: F, t1444: F, t1868: F, t120956: F, t1414: F, t828: F, t121090: F, t27888: F) -> (F, F, F, F, F, F, F, F, F) {
    let t125573 = t32284 * t120962 * t5705;
    let t125576 = t32284 * t120962 * t5696;
    let t125578 = t120952 * t1885;
    let t125580 = t32284 * t5661;
    let t125582 = t121107 * t5665;
    let t125584 = t121110 * t5665;
    let t125587 = t1868 * t1444;
    let t125590 = t120956 * t1414 * t828 * t125587;
    let t125594 = t121090 * t27888;
    (t125573, t125576, t125578, t125580, t125582, t125584, t125587, t125590, t125594)
}
