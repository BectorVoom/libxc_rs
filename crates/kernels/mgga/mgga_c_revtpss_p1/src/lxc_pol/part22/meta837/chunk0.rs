//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2964/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2964<F: Float>(t1412: F, t808: F, t13927: F, t48862: F, t1389: F, t14224: F, t46835: F, t13769: F, t2453: F, t547: F, t9794: F, t14230: F, t2735: F, t46801: F) -> (F, F, F, F, F) {
    let t48863 = t808 * t1412;
    let t48865 = t48862 * t48863 * t13927;
    let t48868 = t46835 * t1389 * t14224;
    let t48872 = t2453 * t547 * t9794 * t13769;
    let t48876 = t2735 * t46801 * t1389 * t14230;
    (t48863, t48865, t48868, t48872, t48876)
}
