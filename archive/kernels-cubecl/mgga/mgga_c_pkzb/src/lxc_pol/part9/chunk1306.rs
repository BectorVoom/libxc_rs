//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1306/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1306<F: Float>(t1167: F, t18655: F, t18659: F, t18668: F, t18940: F, t18963: F, t18967: F, t19140: F, t22445: F, t22452: F, t22461: F, t22469: F, t22475: F, t22913: F, t22921: F, t22924: F, t22927: F, t22933: F, t22936: F, t3185: F, t3235: F, t3236: F, t405: F, t406: F, t6106: F, t6406: F, t6417: F, t6464: F, t6520: F, t6528: F, t758: F, t8380: F, t918: F, t921: F) -> F {
    let t22938 = -F::cast_from(0.17149607247227894789e-2_f64) * t22445 + F::cast_from(0.12862205435420921092e-2_f64) * t3185 * t406 * t8380 * t6417 - F::cast_from(0.85748036236139473944e-3_f64) * t22452 + F::cast_from(0.25724410870841842184e-1_f64) * t3235 * t758 * t19140 * t1167 * t6406 + F::cast_from(0.85748036236139473944e-3_f64) * t18655 + F::cast_from(0.25724410870841842183e-2_f64) * t18659 - F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t22461 + F::cast_from(0.14291339372689912324e-3_f64) * t18668 + F::cast_from(0.12862205435420921092e-2_f64) * t3235 * t758 * t3236 * t6106 - F::cast_from(0.85748036236139473945e-3_f64) * t22469 + F::cast_from(0.25724410870841842183e-2_f64) * t18940 + t22475 + F::cast_from(0.21437009059034868486e-3_f64) * t918 * t758 * t405 * t22913 * t921 - F::cast_from(0.68598428988911579154e-2_f64) * t22921 * t6520 + F::cast_from(0.68598428988911579154e-2_f64) * t22924 * t6528 - F::cast_from(0.11433071498151929859e-2_f64) * t22927 * t6464 - F::cast_from(0.42874018118069736972e-3_f64) * t18963 + F::cast_from(0.85748036236139473944e-3_f64) * t18967 - F::cast_from(0.42874018118069736972e-3_f64) * t22933 - F::cast_from(0.17149607247227894789e-2_f64) * t22936;
    t22938
}
