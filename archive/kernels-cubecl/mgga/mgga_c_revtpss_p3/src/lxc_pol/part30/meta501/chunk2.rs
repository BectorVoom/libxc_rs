//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1870/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1870<F: Float>(t1203: F, t7627: F, t7637: F, t1294: F, t7652: F, t12626: F, t2147: F) -> (F, F, F) {
    let t26940 = t7627 * t1203;
    let t26941 = t7637 * t26940;
    let t26944 = t7627 * t1294;
    let t26945 = t7652 * t26944;
    let t26948 = t2147 * t12626;
    (t26941, t26945, t26948)
}
