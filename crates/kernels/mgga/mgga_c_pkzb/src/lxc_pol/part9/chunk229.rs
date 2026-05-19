//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 229/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk229<F: Float>(t237: F, t248: F, t661: F, t687: F, t690: F, t695: F, t704: F, t710: F, t714: F, t723: F, t252: F) -> (F, F, F) {
    let t727 = t237 * (-F::new(0.310907e-1) * t690 * t248 + F::new(1.0) * t695 * t704 + t661 - t687 - F::cast_from(0.19751673498613801407e-1_f64) * t710 + F::cast_from(0.5848223622634646207e0_f64) * t714 * t723);
    let t729 = F::cast_from(0.19751673498613801407e-1_f64) * t237 * t710;
    let t730 = t237 * t252;
    (t727, t729, t730)
}
