//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 994/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk994<F: Float>(t12012: F, t986: F, t47008: F, t544: F, t11392: F, t12054: F, t12881: F, t1415: F, t1424: F, t1646: F, t2386: F, t2862: F, t2875: F, t46091: F, t46093: F, t46097: F, t46098: F, t46102: F, t46106: F, t46118: F, t46119: F, t46125: F, t46127: F, t46128: F, t46129: F, t46131: F, t46138: F, t48171: F, t48187: F) -> F {
    let t50544 = t12012 * t986;
    let t50549 = t544 * t47008 * t986;
    let t50556 = -F::cast_from(0.21450293971110256002e1_f64) * t48187 * t12881 - F::cast_from(0.21450293971110256002e1_f64) * t12054 * t11392 - t46091 + t46093 - t46097 - F::cast_from(0.44688112439813033337e-1_f64) * t46098 - t46102 + t46106 - t46118 + F::cast_from(0.89376224879626066674e-1_f64) * t46119 - t46125 + t46127 - t46128 - t46129 - t46131 - F::cast_from(0.9585731488480187419e0_f64) * t46138 + F::cast_from(0.85801175884441024008e1_f64) * t48171 * t2862 - F::cast_from(0.71500979903700853338e0_f64) * t1415 * t50544 * t1646 - F::cast_from(0.50050685932590597338e1_f64) * t50549 * t2386 - F::cast_from(0.79445533226334281487e-1_f64) * t544 * t12012 * t2875 * t1424;
    t50556
}
