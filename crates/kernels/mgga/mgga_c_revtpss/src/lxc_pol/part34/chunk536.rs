//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 536/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk536<F: Float>(t1412: F, t1868: F, t1883: F, t221: F, t4019: F, t4018: F, t241: F, t4000: F, t820: F, t550: F, t72: F, t245: F, t125: F, t1882: F, t1873: F, t3957: F) -> (F, F, F, F, F, F, F) {
    let t5651 = t1412 * t1868;
    let t5665 = t4019 * t221 * t1883;
    let t5666 = t4018 * t5665;
    let t5671 = t820 * t4000 * t241;
    let t5672 = t550 * t72;
    let t5673 = t5672 * t245;
    let t5674 = t125 * t1882;
    let t5681 = t3957 * t1873;
    (t5651, t5665, t5666, t5671, t5673, t5674, t5681)
}
