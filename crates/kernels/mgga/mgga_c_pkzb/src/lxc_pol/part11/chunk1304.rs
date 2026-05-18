//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1304/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1304<F: Float>(t10169: F, t11180: F, t18520: F, t898: F, t31333: F, t31335: F, t31337: F, t31339: F, t31369: F, t31372: F, t31375: F, t31377: F, t31380: F, t31383: F) -> (F, F) {
    let t31640 = F::new(0.12304822629859687989e5) * t898 * t18520 * t11180 * t10169;
    let t31641 = t31333 - t31335 + t31337 + t31339 - t31369 - t31372 - t31375 + t31377 + t31380 + t31383 + t31640;
    (t31640, t31641)
}
