//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1341/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1341<F: Float>(t21881: F, t94: F, t1518: F, t4245: F, t1501: F, t4292: F, t93: F, t10208: F, t625: F, t46157: F, t69: F, t2289: F, t2339: F, t8260: F, t655: F, t8269: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t108714 = t94 * t21881;
    let t109150 = t4245 * t1518;
    let t109153 = t1501 * t4292;
    let t109242 = t93 * t21881;
    let t116912 = t625 * t10208;
    let t116919 = t69 * t46157;
    let t116926 = t2289 * t2339;
    let t116927 = t116926 * t8260;
    let t116929 = t2289 * t655;
    let t116930 = t116929 * t8269;
    (t108714, t109150, t109153, t109242, t116912, t116919, t116926, t116927, t116929, t116930)
}
