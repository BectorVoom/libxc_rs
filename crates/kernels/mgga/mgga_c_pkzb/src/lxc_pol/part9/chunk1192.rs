//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1192/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1192<F: Float>(t23007: F, t3174: F, t68: F, t8269: F, t8281: F, t10063: F, t1167: F, t2226: F, t22945: F, t22947: F, t22951: F, t22952: F, t22957: F, t22966: F, t22972: F, t22973: F, t22974: F, t22979: F, t22980: F, t22989: F, t22991: F, t2380: F, t2381: F, t2888: F, t3186: F, t3206: F, t3207: F, t3265: F, t406: F, t6366: F, t6367: F, t6406: F, t6424: F, t6480: F, t6506: F, t8319: F, t8409: F, t8418: F, t8435: F, t8436: F, t8450: F, t8451: F, t921: F) -> (F,) {
    let t23008 = t23007 / 72.0;
    let t23010 = t3174 * t68 * t8269;
    let t23013 = t3174 * t68 * t8281;
    let t23017 = 0.38586616306262763275e-2 * t2380 * t6366 * t3265 * t6367 + t22945 - 0.85748036236139473944e-3 * t22947 + t22951 - 0.64311027177104605458e-3 * t3206 * t406 * t22952 * t3207 - 0.38586616306262763275e-2 * t8435 * t406 * t22957 * t8436 + 0.64311027177104605458e-3 * t8450 * t406 * t22957 * t8451 - 0.21437009059034868486e-3 * t3206 * t406 * t3186 * t22966 + 0.30011812682648815881e-2 * t22972 * t406 * t22973 * t22974 - 0.21437009059034868486e-3 * t22979 * t406 * t22973 * t22980 + 0.68598428988911579154e-2 * t8319 * t6480 - t22989 - 0.85748036236139473944e-3 * t22991 - 0.42874018118069736972e-3 * t2380 * t2381 * t1167 * t6506 * t921 - 3.0 / 16.0 * t3174 * t2888 * t8409 * t2226 + t3174 * t2888 * t8418 * t6406 / 4.0 - t23008 + t23010 / 48.0 - t23013 / 16.0 - t10063 * t6424 / 6.0;
    (t23017,)
}
