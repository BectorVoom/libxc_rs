//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 684/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk684<F: Float>(t342: F, t4980: F, t3154: F, t3302: F, t3316: F, t378: F, t1043: F, t357: F, t198: F, t336: F) -> (F, F, F, F, F, F) {
    let t4981 = t342 * t4980;
    let t4982 = t3302 * t3154;
    let t4995 = t3316 * t378;
    let t4996 = t342 * t4995;
    let t4997 = t3302 * t1043;
    let t4998 = t4997 * t357;
    let t5023 = t198 * t336;
    (t4981, t4982, t4995, t4996, t4998, t5023)
}
