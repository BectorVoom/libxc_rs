//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1170/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1170<F: Float>(t1165: F, t1180: F, t12995: F, t12999: F, t1552: F, t16174: F, t16185: F, t16191: F, t16203: F, t16205: F, t16207: F, t16319: F, t21128: F, t21136: F, t21141: F, t21146: F) -> F {
    let t21157 = F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1165 * t1552 * t21128 - F::cast_from(0.34299214494455789578e-2_f64) * t21136 - F::cast_from(0.17149607247227894789e-2_f64) * t21141 - F::cast_from(0.34299214494455789578e-1_f64) * t21146 + F::cast_from(0.25724410870841842183e-2_f64) * t16174 + F::cast_from(0.68598428988911579156e-2_f64) * t12995 - F::cast_from(0.10289764348336736874e-1_f64) * t12999 + F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t16185 - F::cast_from(0.17149607247227894789e-2_f64) * t16191 + F::cast_from(0.85748036236139473944e-3_f64) * t16203 - F::cast_from(0.17149607247227894789e-2_f64) * t16205 - F::cast_from(0.85748036236139473944e-3_f64) * t16207 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t16319;
    t21157
}
