//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1471/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1471<F: Float>(t2195: F, t2289: F, t31027: F, t8312: F, t31032: F, t8316: F, t104: F, t2357: F, t116: F, t8320: F, t10199: F, t655: F) -> (F, F, F, F, F, F) {
    let t31134 = F::new(11.0) / F::new(9.0) * t2289 * t2195;
    let t31135 = t31027 * t8312;
    let t31137 = t31032 * t8316;
    let t31149 = t104 * t2357;
    let t31234 = t116 * t8320;
    let t31287 = t10199 * t655;
    (t31134, t31135, t31137, t31149, t31234, t31287)
}
