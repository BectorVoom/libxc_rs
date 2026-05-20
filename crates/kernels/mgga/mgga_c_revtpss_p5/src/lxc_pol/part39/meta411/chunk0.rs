//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1488/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1488<F: Float>(t1464: F, t8283: F, t10208: F, t625: F, t31036: F, t31027: F, t31040: F, t31032: F, t31059: F, t46157: F, t69: F, t2289: F, t2339: F) -> (F, F, F, F, F, F, F) {
    let t116899 = t8283 * t1464;
    let t116912 = t625 * t10208;
    let t116913 = t116912 * t31036;
    let t116915 = t31027 * t31040;
    let t116917 = t31032 * t31059;
    let t116919 = t69 * t46157;
    let t116926 = t2289 * t2339;
    (t116899, t116912, t116913, t116915, t116917, t116919, t116926)
}
