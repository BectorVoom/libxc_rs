//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1104/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1104<F: Float>(t15127: F, t2852: F, t4186: F, t606: F, t2850: F, t128: F) -> (F, F, F) {
    let t15128 = 0.13418888888888888889e0 * t15127;
    let t15129 = t2852 * t4186;
    let t15130 = t15129 * t606;
    let t15131 = t2850 * t15130;
    let t15132 = t128 * t15131;
    (t15128, t15130, t15132)
}
