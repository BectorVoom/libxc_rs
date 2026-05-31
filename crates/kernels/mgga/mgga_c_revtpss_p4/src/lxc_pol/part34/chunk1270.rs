//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1270/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1270<F: Float>(t30128: F, t7732: F, t1936: F, t25043: F, t651: F, t2014: F, t28172: F, t29494: F, t109173: F, t7900: F, t1583: F, t5966: F) -> (F, F, F, F, F) {
    let t113086 = F::cast_from(6.0_f64) * t7732 * t30128;
    let t113089 = F::cast_from(2.0_f64) * t651 * t25043 * t1936;
    let t113092 = F::cast_from(9.0_f64) * t2014 * t28172 * t29494;
    let t113095 = F::cast_from(9.0_f64) * t2014 * t109173 * t7900;
    let t113096 = t5966 * t1583;
    (t113086, t113089, t113092, t113095, t113096)
}
