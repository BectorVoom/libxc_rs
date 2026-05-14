//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 692/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk692<F: Float>(t2148: F, t8931: F, t2150: F, t473: F, t2147: F, t456: F, t3565: F) -> (F, F, F, F) {
    let t8932 = t2148 * t8931;
    let t8933 = t2150 * t473;
    let t8936 = t2147 * t456;
    let t8937 = t8936 * t3565;
    (t8932, t8933, t8936, t8937)
}
