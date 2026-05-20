//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2305/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2305<F: Float>(t1043: F, t3155: F, t12131: F, t357: F, t1651: F, t905: F, t16509: F, t4891: F, t16584: F, t1062: F, t15670: F, t1668: F, t3181: F) -> (F, F, F, F, F, F, F) {
    let t19634 = t3155 * t1043;
    let t19639 = t12131 * t357;
    let t19705 = t1651 * t905;
    let t19738 = t16509 * t4891;
    let t19741 = t16584 * t4891;
    let t19878 = t15670 * t1062;
    let t19979 = t3181 * t1668;
    (t19634, t19639, t19705, t19738, t19741, t19878, t19979)
}
