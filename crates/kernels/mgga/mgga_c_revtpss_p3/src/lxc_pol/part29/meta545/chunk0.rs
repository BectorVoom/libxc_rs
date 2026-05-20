//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1881/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1881<F: Float>(t26072: F, t26292: F, t7493: F, t9292: F, t136: F, t137: F, t2097: F, t94386: F, t94391: F, t1358: F, t212: F, t26333: F, t689: F) -> (F, F, F, F, F, F) {
    let t96211 = t26072 * t26292;
    let t96218 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t7493;
    let t96220 = t2097 * t136 * t137;
    let t96221 = t96220 * t94386;
    let t96222 = t94391 * t96221;
    let t96226 = t689 * t212 * t26333 * t1358;
    (t96211, t96218, t96220, t96221, t96222, t96226)
}
