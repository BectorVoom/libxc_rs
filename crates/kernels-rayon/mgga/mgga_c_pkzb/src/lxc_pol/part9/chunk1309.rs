//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1309/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1309(t22988: f64, t2380: f64, t6475: f64, t8463: f64, t3174: f64, t3176: f64, t487: f64, t68: f64, t8269: f64, t8281: f64, t10063: f64, t1167: f64, t2226: f64, t22945: f64, t22947: f64, t22951: f64, t22952: f64, t22957: f64, t22966: f64, t22972: f64, t22973: f64, t22974: f64, t22979: f64, t22980: f64, t2381: f64, t2888: f64, t3186: f64, t3206: f64, t3207: f64, t3265: f64, t406: f64, t6366: f64, t6367: f64, t6406: f64, t6424: f64, t6480: f64, t6506: f64, t8319: f64, t8409: f64, t8418: f64, t8435: f64, t8436: f64, t8450: f64, t8451: f64, t921: f64) -> f64 {
    let t22989 = 0.28582678745379824648e-3_f64 * t22988;
    let t22991 = t2380 * t6475 * t8463;
    let t23007 = t3174 * t487 * t3176;
    let t23008 = t23007 / 72.0_f64;
    let t23010 = t3174 * t68 * t8269;
    let t23013 = t3174 * t68 * t8281;
    let t23017 = 0.38586616306262763275e-2_f64 * t2380 * t6366 * t3265 * t6367 + t22945 - 0.85748036236139473944e-3_f64 * t22947 + t22951 - 0.64311027177104605458e-3_f64 * t3206 * t406 * t22952 * t3207 - 0.38586616306262763275e-2_f64 * t8435 * t406 * t22957 * t8436 + 0.64311027177104605458e-3_f64 * t8450 * t406 * t22957 * t8451 - 0.21437009059034868486e-3_f64 * t3206 * t406 * t3186 * t22966 + 0.30011812682648815881e-2_f64 * t22972 * t406 * t22973 * t22974 - 0.21437009059034868486e-3_f64 * t22979 * t406 * t22973 * t22980 + 0.68598428988911579154e-2_f64 * t8319 * t6480 - t22989 - 0.85748036236139473944e-3_f64 * t22991 - 0.42874018118069736972e-3_f64 * t2380 * t2381 * t1167 * t6506 * t921 - 3.0_f64 / 16.0_f64 * t3174 * t2888 * t8409 * t2226 + t3174 * t2888 * t8418 * t6406 / 4.0_f64 - t23008 + t23010 / 48.0_f64 - t23013 / 16.0_f64 - t10063 * t6424 / 6.0_f64;
    t23017
}
