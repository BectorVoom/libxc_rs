//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1189/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1189<F: Float>(t22919: F, t46: F, t6515: F, t6524: F, t6456: F, t3206: F, t8446: F, t926: F, t2380: F, t6475: F, t8459: F, t1167: F, t18655: F, t18659: F, t18668: F, t18940: F, t18963: F, t18967: F, t19140: F, t22445: F, t22452: F, t22461: F, t22469: F, t22475: F, t22913: F, t3185: F, t3235: F, t3236: F, t405: F, t406: F, t6106: F, t6406: F, t6417: F, t6464: F, t6520: F, t6528: F, t758: F, t8380: F, t918: F, t921: F) -> (F,) {
    let t22920 = t22919 * t46;
    let t22921 = t6515 * t22920;
    let t22924 = t6524 * t22920;
    let t22927 = t6456 * t22920;
    let t22933 = t3206 * t926 * t8446;
    let t22936 = t2380 * t6475 * t8459;
    let t22938 = -0.17149607247227894789e-2 * t22445 + 0.12862205435420921092e-2 * t3185 * t406 * t8380 * t6417 - 0.85748036236139473944e-3 * t22452 + 0.25724410870841842184e-1 * t3235 * t758 * t19140 * t1167 * t6406 + 0.85748036236139473944e-3 * t18655 + 0.25724410870841842183e-2 * t18659 - 5.0 / 486.0 * t22461 + 0.14291339372689912324e-3 * t18668 + 0.12862205435420921092e-2 * t3235 * t758 * t3236 * t6106 - 0.85748036236139473945e-3 * t22469 + 0.25724410870841842183e-2 * t18940 + t22475 + 0.21437009059034868486e-3 * t918 * t758 * t405 * t22913 * t921 - 0.68598428988911579154e-2 * t22921 * t6520 + 0.68598428988911579154e-2 * t22924 * t6528 - 0.11433071498151929859e-2 * t22927 * t6464 - 0.42874018118069736972e-3 * t18963 + 0.85748036236139473944e-3 * t18967 - 0.42874018118069736972e-3 * t22933 - 0.17149607247227894789e-2 * t22936;
    (t22938,)
}
