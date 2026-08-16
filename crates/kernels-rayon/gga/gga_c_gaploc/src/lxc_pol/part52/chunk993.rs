//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 993/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk993(t1: f64, t106: f64, t14266: f64, t192: f64, t12054: f64, t12113: f64, t12117: f64, t12881: f64, t1339: f64, t14271: f64, t14298: f64, t1441: f64, t1445: f64, t1450: f64, t1537: f64, t1562: f64, t1572: f64, t2877: f64, t3702: f64, t4130: f64, t42296: f64, t447: f64, t46060: f64, t46064: f64, t46068: f64, t46072: f64, t46073: f64, t46078: f64, t46079: f64, t4614: f64, t4673: f64, t4781: f64, t48190: f64, t493: f64, t49873: f64, t49921: f64, t536: f64, t590: f64, t8063: f64, t8072: f64) -> f64 {
    let t50522 = t14266 * t1 * t106 * t192;
    let t50533 = -t46060 - 0.18404604457881959845e2_f64 * t1562 * t4614 * t14298 - 0.14300195980740170668e1_f64 * t12054 * t42296 + 0.47667319935800568892e0_f64 * t3702 * t8063 + 0.95334639871601137787e0_f64 * t1572 * t4673 * t14271 - 0.23005755572352449806e1_f64 * t1450 * t1445 * t49873 * t447 + 0.30674340763136599742e1_f64 * t4781 * t4130 * t49921 * t590 + 0.20449560508757733161e1_f64 * t1441 * t493 * t49921 * t590 - 0.51123901271894332902e1_f64 * t1537 * t1339 * t49921 * t590 + t46064 - t46068 - t46072 + t46073 - t46078 + t46079 + 0.35750489951850426669e0_f64 * t536 * t50522 + 0.71500979903700853338e0_f64 * t12113 * t2877 + 0.71500979903700853338e0_f64 * t12117 * t2877 + 0.71500979903700853338e0_f64 * t3702 * t8072 - 0.21450293971110256002e1_f64 * t48190 * t12881;
    t50533
}
