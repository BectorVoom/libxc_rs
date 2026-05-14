//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1068/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1068<F: Float>(t94669: F, t96271: F, t26277: F, t94913: F, t25944: F, t96259: F, t1385: F, t7506: F, t10073: F, t25937: F, t7282: F, t26069: F, t96255: F, t2453: F, t3908: F, t7507: F) -> (F, F, F, F, F, F, F) {
    let t96378 = t94669 * t96271;
    let t96380 = t94913 * t26277;
    let t96382 = t25944 * t96259;
    let t96392 = t1385 * t7506;
    let t96398 = t10073 * t7282 * t25937 * t7506;
    let t96401 = 0.91399340044406952588e-2 * t26069 * t96255;
    let t96403 = t2453 * t7507 * t3908;
    (t96378, t96380, t96382, t96392, t96398, t96401, t96403)
}
