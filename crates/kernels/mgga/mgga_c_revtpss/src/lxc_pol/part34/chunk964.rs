//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 964/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk964<F: Float>(t2723: F, t6016: F, t1558: F, t5977: F, t10871: F, t231: F, t10552: F, t10554: F, t23096: F, t23097: F, t23102: F, t23103: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F) {
    let t23160 = t2723 * t6016;
    let t23167 = t5977 * t1558;
    let t23168 = t23167 * t10871;
    let t23172 = t23167 * t2723;
    let t23177 = t23167 * t231;
    let t23185 = t23096 - t9278 + t9308 + t9316 + t9329 + t9333 + t23097 - t10552 + t10554 + t23102 + t23103;
    (t23160, t23167, t23168, t23172, t23177, t23185)
}
