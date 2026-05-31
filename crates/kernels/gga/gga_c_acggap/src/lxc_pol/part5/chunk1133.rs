//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1133/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1133<F: Float>(t1846: F, t3765: F, t1851: F, t3237: F, t1008: F, t5561: F, t1077: F, t12458: F, t12460: F, t1426: F, t15348: F, t15350: F, t15362: F, t15366: F, t15370: F, t15378: F, t1713: F, t368: F, t418: F) -> F {
    let t20280 = t3765 * t1846;
    let t20286 = t3237 * t1851;
    let t20290 = t1008 * t5561;
    let t20298 = F::cast_from(0.51448821741683684367e-2_f64) * t15348 - F::cast_from(0.56688979511669985553e-2_f64) * t20280 - F::cast_from(0.34013387707001991332e-1_f64) * t15350 + F::cast_from(0.25724410870841842183e-2_f64) * t15362 + F::cast_from(0.34299214494455789578e-2_f64) * t15366 + F::cast_from(0.17149607247227894789e-2_f64) * t15370 - F::cast_from(0.32012600194825403606e-1_f64) * t20286 + F::cast_from(0.11337795902333997111e-1_f64) * t12458 - F::cast_from(0.56688979511669985553e-2_f64) * t12460 + F::cast_from(0.17149607247227894789e-1_f64) * t20290 + F::cast_from(0.85748036236139473944e-2_f64) * t418 * t1426 * t368 * t1713 * t1077 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t15378;
    t20298
}
