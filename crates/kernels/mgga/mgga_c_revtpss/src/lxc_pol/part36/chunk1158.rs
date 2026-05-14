//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1158/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1158<F: Float>(t30005: F, t7732: F, t30128: F, t1936: F, t25043: F, t651: F, t2014: F, t28172: F, t29494: F, t109173: F, t7900: F, t1583: F, t5966: F, t25207: F, t23279: F, t27159: F) -> (F, F, F, F, F, F, F, F) {
    let t113084 = 6.0 * t7732 * t30005;
    let t113086 = 6.0 * t7732 * t30128;
    let t113089 = 2.0 * t651 * t25043 * t1936;
    let t113092 = 9.0 * t2014 * t28172 * t29494;
    let t113095 = 9.0 * t2014 * t109173 * t7900;
    let t113096 = t5966 * t1583;
    let t113097 = t25207 * t113096;
    let t113100 = t27159 * t23279;
    (t113084, t113086, t113089, t113092, t113095, t113096, t113097, t113100)
}
