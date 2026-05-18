//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 555/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk555<F: Float>(t550: F, t72: F, t245: F, t125: F, t1882: F, t1873: F, t3957: F, t1892: F, t213: F, t1357: F, t1904: F, t689: F) -> (F, F, F, F, F, F) {
    let t5672 = t550 * t72;
    let t5673 = t5672 * t245;
    let t5674 = t125 * t1882;
    let t5681 = t3957 * t1873;
    let t5715 = t213 * t1892;
    let t5718 = t1357 * t1904;
    let t5719 = t689 * t5718;
    (t5673, t5674, t5681, t5715, t5718, t5719)
}
