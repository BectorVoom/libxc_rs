//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 860/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk860<F: Float>(t1: F, t106: F, t14266: F, t192: F, t12054: F, t12113: F, t12117: F, t12881: F, t1339: F, t14271: F, t14298: F, t1441: F, t1445: F, t1450: F, t1537: F, t1562: F, t1572: F, t2877: F, t3702: F, t4130: F, t42296: F, t447: F, t46060: F, t46064: F, t46068: F, t46072: F, t46073: F, t46078: F, t46079: F, t4614: F, t4673: F, t4781: F, t48190: F, t493: F, t49873: F, t49921: F, t536: F, t590: F, t8063: F, t8072: F) -> (F,) {
    let t50522 = t14266 * t1 * t106 * t192;
    let t50533 = -t46060 - 0.18404604457881959845e2 * t1562 * t4614 * t14298 - 0.14300195980740170668e1 * t12054 * t42296 + 0.47667319935800568892e0 * t3702 * t8063 + 0.95334639871601137787e0 * t1572 * t4673 * t14271 - 0.23005755572352449806e1 * t1450 * t1445 * t49873 * t447 + 0.30674340763136599742e1 * t4781 * t4130 * t49921 * t590 + 0.20449560508757733161e1 * t1441 * t493 * t49921 * t590 - 0.51123901271894332902e1 * t1537 * t1339 * t49921 * t590 + t46064 - t46068 - t46072 + t46073 - t46078 + t46079 + 0.35750489951850426669e0 * t536 * t50522 + 0.71500979903700853338e0 * t12113 * t2877 + 0.71500979903700853338e0 * t12117 * t2877 + 0.71500979903700853338e0 * t3702 * t8072 - 0.21450293971110256002e1 * t48190 * t12881;
    (t50533,)
}
