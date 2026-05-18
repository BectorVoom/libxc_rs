//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1230/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1230<F: Float>(t17351: F, t17455: F, t20705: F, t20787: F, t25633: F, t25636: F, t30284: F, t30287: F, t665: F, t672: F, t3532: F, t667: F) -> (F, F, F) {
    let t30288 = t17455 - F::new(28.0) / F::new(27.0) * t17351 - F::new(28.0) / F::new(9.0) * t20705 + t20787 + F::new(4.0) / F::new(3.0) * t25633 - t25636 - t30284 / F::new(3.0) + t30287;
    let t30289 = t665 * t30288;
    let t30291 = t672 * t30288;
    let t30293 = t3532 * t667;
    (t30289, t30291, t30293)
}
