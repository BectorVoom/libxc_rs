//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1051/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1051<F: Float>(t513: F, t930: F, t1165: F, t1532: F, t3194: F, t322: F, t6258: F, t1748: F, t879: F, t1444: F, t1181: F, t4282: F, t530: F, t1180: F, t12995: F, t12999: F, t1552: F, t16174: F, t16185: F, t16191: F, t16203: F, t16205: F, t16207: F, t16319: F) -> (F, F, F) {
    let t21128 = t930 * t513;
    let t21136 = t3194 * t1165 * t1532 * t6258 * t322;
    let t21141 = t3194 * t1165 * t1532 * t1748 * t879;
    let t21143 = t1444 * t322;
    let t21146 = t4282 * t1181 * t530 * t21143;
    let t21157 = 0.85748036236139473944e-3 * t1180 * t1165 * t1552 * t21128 - 0.34299214494455789578e-2 * t21136 - 0.17149607247227894789e-2 * t21141 - 0.34299214494455789578e-1 * t21146 + 0.25724410870841842183e-2 * t16174 + 0.68598428988911579156e-2 * t12995 - 0.10289764348336736874e-1 * t12999 + 7.0 / 12.0 * t16185 - 0.17149607247227894789e-2 * t16191 + 0.85748036236139473944e-3 * t16203 - 0.17149607247227894789e-2 * t16205 - 0.85748036236139473944e-3 * t16207 + 7.0 / 144.0 * t16319;
    (t21128, t21143, t21157)
}
