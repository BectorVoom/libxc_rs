//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1196/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1196<F: Float>(t20743: F, t208: F, t218: F, t219: F, t20716: F, t17351: F, t17354: F, t17357: F, t17455: F, t20705: F, t20719: F, t20745: F) -> (F, F) {
    let t20781 = t218 * t219 * t208 * t20743;
    let t20787 = F::new(4.0) / F::new(3.0) * t20716;
    let t20788 = t17455 - F::new(28.0) / F::new(9.0) * t17351 + F::new(4.0) / F::new(3.0) * t17354 - t17357 / F::new(3.0) - F::new(28.0) / F::new(27.0) * t20705 + t20787 - t20719 + t20745;
    (t20781, t20788)
}
