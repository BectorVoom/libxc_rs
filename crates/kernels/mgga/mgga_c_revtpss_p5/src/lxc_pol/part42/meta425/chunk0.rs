//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1487/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1487<F: Float>(t1501: F, t4292: F, t21881: F, t93: F, t10208: F, t625: F, t46157: F, t69: F, t2289: F, t2339: F, t655: F, t10199: F, t2195: F) -> (F, F, F, F, F, F, F) {
    let t109153 = t1501 * t4292;
    let t109242 = t93 * t21881;
    let t116912 = t625 * t10208;
    let t116919 = t69 * t46157;
    let t116926 = t2289 * t2339;
    let t116929 = t2289 * t655;
    let t117183 = F::new(154.0) / F::new(27.0) * t10199 * t2195;
    (t109153, t109242, t116912, t116919, t116926, t116929, t117183)
}
