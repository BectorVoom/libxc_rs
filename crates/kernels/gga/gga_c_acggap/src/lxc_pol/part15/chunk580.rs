//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 580/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk580<F: Float>(t1137: F, t1319: F, t1008: F, t1446: F, t1451: F, t3228: F, t542: F, t1588: F, t537: F, t1576: F, t1298: F, t322: F) -> (F, F, F, F, F, F, F, F) {
    let t5175 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t1137 * t1319;
    let t5222 = F::cast_from(0.34299214494455789578e-2_f64) * t1008 * t1446;
    let t5224 = F::cast_from(0.17149607247227894789e-2_f64) * t1008 * t1451;
    let t5226 = t3228 * t542;
    let t5229 = F::cast_from(0.85748036236139473944e-3_f64) * t1008 * t1588;
    let t5240 = t3228 * t537;
    let t5243 = F::cast_from(0.85748036236139473944e-3_f64) * t1008 * t1576;
    let t5249 = t1298 * t322;
    (t5175, t5222, t5224, t5226, t5229, t5240, t5243, t5249)
}
