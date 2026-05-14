//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1045/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1045<F: Float>(t30: F, t6079: F, t1468: F, t1583: F, t6075: F, t1940: F, t1963: F, t2403: F, t25206: F, t25445: F, t27368: F, t29592: F, t29599: F, t29602: F, t29606: F, t29705: F, t4541: F, t5824: F, t7091: F, t7749: F, t7783: F, t7787: F) -> (F, F, F, F) {
    let t29713 = t30 * t6079;
    let t29716 = t1468 * t1583;
    let t29719 = t30 * t6075;
    let t29726 = 3.0 * t4541 * t29592 + 3.0 * t2403 * t7783 * t7749 - 3.0 * t25206 * t29599 + 3.0 * t2403 * t1963 * t29602 + 3.0 / 2.0 * t2403 * t1963 * t29606 + t1940 * t29705 * t30 / 2.0 - t1940 * t27368 * t7787 + t1940 * t7783 * t1468 + t1940 * t25445 * t29713 - t1940 * t7091 * t29716 - t1940 * t7091 * t29719 / 2.0 + t1940 * t1963 * t5824 / 2.0;
    (t29713, t29716, t29719, t29726)
}
