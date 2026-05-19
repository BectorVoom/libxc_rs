//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1309/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1309<F: Float>(t22988: F, t2380: F, t6475: F, t8463: F, t3174: F, t3176: F, t487: F, t68: F, t8269: F, t8281: F, t10063: F, t1167: F, t2226: F, t22945: F, t22947: F, t22951: F, t22952: F, t22957: F, t22966: F, t22972: F, t22973: F, t22974: F, t22979: F, t22980: F, t2381: F, t2888: F, t3186: F, t3206: F, t3207: F, t3265: F, t406: F, t6366: F, t6367: F, t6406: F, t6424: F, t6480: F, t6506: F, t8319: F, t8409: F, t8418: F, t8435: F, t8436: F, t8450: F, t8451: F, t921: F) -> F {
    let t22989 = F::cast_from(0.28582678745379824648e-3_f64) * t22988;
    let t22991 = t2380 * t6475 * t8463;
    let t23007 = t3174 * t487 * t3176;
    let t23008 = t23007 / F::new(72.0);
    let t23010 = t3174 * t68 * t8269;
    let t23013 = t3174 * t68 * t8281;
    let t23017 = F::cast_from(0.38586616306262763275e-2_f64) * t2380 * t6366 * t3265 * t6367 + t22945 - F::cast_from(0.85748036236139473944e-3_f64) * t22947 + t22951 - F::cast_from(0.64311027177104605458e-3_f64) * t3206 * t406 * t22952 * t3207 - F::cast_from(0.38586616306262763275e-2_f64) * t8435 * t406 * t22957 * t8436 + F::cast_from(0.64311027177104605458e-3_f64) * t8450 * t406 * t22957 * t8451 - F::cast_from(0.21437009059034868486e-3_f64) * t3206 * t406 * t3186 * t22966 + F::cast_from(0.30011812682648815881e-2_f64) * t22972 * t406 * t22973 * t22974 - F::cast_from(0.21437009059034868486e-3_f64) * t22979 * t406 * t22973 * t22980 + F::cast_from(0.68598428988911579154e-2_f64) * t8319 * t6480 - t22989 - F::cast_from(0.85748036236139473944e-3_f64) * t22991 - F::cast_from(0.42874018118069736972e-3_f64) * t2380 * t2381 * t1167 * t6506 * t921 - F::new(3.0) / F::new(16.0) * t3174 * t2888 * t8409 * t2226 + t3174 * t2888 * t8418 * t6406 / F::new(4.0) - t23008 + t23010 / F::new(48.0) - t23013 / F::new(16.0) - t10063 * t6424 / F::new(6.0);
    t23017
}
