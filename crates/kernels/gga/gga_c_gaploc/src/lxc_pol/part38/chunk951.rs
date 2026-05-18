//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 951/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk951<F: Float>(t10525: F, t10526: F, t1441: F, t2487: F, t44404: F, t46052: F, t46055: F, t46057: F, t46060: F, t46064: F, t46068: F, t46072: F, t46073: F, t46078: F, t46079: F, t46080: F, t46084: F, t46091: F, t46093: F, t46097: F, t46098: F, t46102: F, t46106: F, t493: F, t590: F, t6711: F, t6963: F, t6964: F) -> F {
    let t46110 = -t46052 + t46055 + t46057 - t46060 + t46064 - t46068 - t46072 + t46073 + F::new(0.20449560508757733161e1) * t1441 * t493 * t44404 * t590 - t46078 + t46079 - F::new(0.42900587942220512004e1) * t10525 * t10526 * t46080 - F::new(0.14300195980740170668e1) * t6963 * t6964 * t46084 - t46091 + t46093 - t46097 - F::new(0.44688112439813033338e-1) * t46098 - t46102 + t46106 + F::new(0.87421871174939309263e2) * t2487 * t6711 * t46080;
    t46110
}
