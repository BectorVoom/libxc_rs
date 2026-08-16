//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 994/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk994(t12012: f64, t986: f64, t47008: f64, t544: f64, t11392: f64, t12054: f64, t12881: f64, t1415: f64, t1424: f64, t1646: f64, t2386: f64, t2862: f64, t2875: f64, t46091: f64, t46093: f64, t46097: f64, t46098: f64, t46102: f64, t46106: f64, t46118: f64, t46119: f64, t46125: f64, t46127: f64, t46128: f64, t46129: f64, t46131: f64, t46138: f64, t48171: f64, t48187: f64) -> f64 {
    let t50544 = t12012 * t986;
    let t50549 = t544 * t47008 * t986;
    let t50556 = -0.21450293971110256002e1_f64 * t48187 * t12881 - 0.21450293971110256002e1_f64 * t12054 * t11392 - t46091 + t46093 - t46097 - 0.44688112439813033337e-1_f64 * t46098 - t46102 + t46106 - t46118 + 0.89376224879626066674e-1_f64 * t46119 - t46125 + t46127 - t46128 - t46129 - t46131 - 0.9585731488480187419e0_f64 * t46138 + 0.85801175884441024008e1_f64 * t48171 * t2862 - 0.71500979903700853338e0_f64 * t1415 * t50544 * t1646 - 0.50050685932590597338e1_f64 * t50549 * t2386 - 0.79445533226334281487e-1_f64 * t544 * t12012 * t2875 * t1424;
    t50556
}
