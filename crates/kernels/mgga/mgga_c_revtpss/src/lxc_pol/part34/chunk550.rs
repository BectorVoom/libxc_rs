//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 550/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk550<F: Float>(t198: F, t532: F, t1907: F, t4147: F, t1317: F, t1857: F, t1320: F, t1468: F, t3833: F, t1711: F, t3841: F, t1856: F, t749: F) -> (F, F, F, F, F, F, F) {
    let t5541 = t198 * t532;
    let t5542 = t1907 * t4147;
    let t5545 = t1317 * t1857;
    let t5547 = t1320 * t1857;
    let t5549 = t3833 * t1468;
    let t5557 = t3841 * t1711;
    let t5569 = t1856 * t749;
    (t5541, t5542, t5545, t5547, t5549, t5557, t5569)
}
