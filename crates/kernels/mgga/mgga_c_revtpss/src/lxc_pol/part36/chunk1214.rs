//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1214/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1214<F: Float>(t7059: F, t9288: F, t7064: F, t25305: F, t92868: F, t7036: F, t820: F, t844: F, t2482: F, t814: F, t228: F, t25273: F) -> (F, F, F, F, F, F) {
    let t92871 = t7059 * t9288;
    let t92873 = F::new(0.39982213492741449076e-1) * t7064 * t92871;
    let t92875 = F::new(0.91399340044406952588e-2) * t25305 * t92868;
    let t92951 = t820 * t7036 * t844;
    let t92955 = t2482 * t7036 * t814;
    let t92968 = t25273 * t228;
    (t92871, t92873, t92875, t92951, t92955, t92968)
}
