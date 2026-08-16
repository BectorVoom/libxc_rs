//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 951/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk951(t10525: f64, t10526: f64, t1441: f64, t2487: f64, t44404: f64, t46052: f64, t46055: f64, t46057: f64, t46060: f64, t46064: f64, t46068: f64, t46072: f64, t46073: f64, t46078: f64, t46079: f64, t46080: f64, t46084: f64, t46091: f64, t46093: f64, t46097: f64, t46098: f64, t46102: f64, t46106: f64, t493: f64, t590: f64, t6711: f64, t6963: f64, t6964: f64) -> f64 {
    let t46110 = -t46052 + t46055 + t46057 - t46060 + t46064 - t46068 - t46072 + t46073 + 0.20449560508757733161e1_f64 * t1441 * t493 * t44404 * t590 - t46078 + t46079 - 0.42900587942220512004e1_f64 * t10525 * t10526 * t46080 - 0.14300195980740170668e1_f64 * t6963 * t6964 * t46084 - t46091 + t46093 - t46097 - 0.44688112439813033338e-1_f64 * t46098 - t46102 + t46106 + 0.87421871174939309263e2_f64 * t2487 * t6711 * t46080;
    t46110
}
