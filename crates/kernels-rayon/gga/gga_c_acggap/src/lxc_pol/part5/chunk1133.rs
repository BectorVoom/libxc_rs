//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1133/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1133(t1846: f64, t3765: f64, t1851: f64, t3237: f64, t1008: f64, t5561: f64, t1077: f64, t12458: f64, t12460: f64, t1426: f64, t15348: f64, t15350: f64, t15362: f64, t15366: f64, t15370: f64, t15378: f64, t1713: f64, t368: f64, t418: f64) -> f64 {
    let t20280 = t3765 * t1846;
    let t20286 = t3237 * t1851;
    let t20290 = t1008 * t5561;
    let t20298 = 0.51448821741683684367e-2_f64 * t15348 - 0.56688979511669985553e-2_f64 * t20280 - 0.34013387707001991332e-1_f64 * t15350 + 0.25724410870841842183e-2_f64 * t15362 + 0.34299214494455789578e-2_f64 * t15366 + 0.17149607247227894789e-2_f64 * t15370 - 0.32012600194825403606e-1_f64 * t20286 + 0.11337795902333997111e-1_f64 * t12458 - 0.56688979511669985553e-2_f64 * t12460 + 0.17149607247227894789e-1_f64 * t20290 + 0.85748036236139473944e-2_f64 * t418 * t1426 * t368 * t1713 * t1077 - 35.0_f64 / 108.0_f64 * t15378;
    t20298
}
