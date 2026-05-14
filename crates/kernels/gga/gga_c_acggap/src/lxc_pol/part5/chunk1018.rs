//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1018/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1018<F: Float>(t1846: F, t3765: F, t1851: F, t3237: F, t1008: F, t5561: F, t1077: F, t12458: F, t12460: F, t1426: F, t15348: F, t15350: F, t15362: F, t15366: F, t15370: F, t15378: F, t1713: F, t368: F, t418: F) -> (F,) {
    let t20280 = t3765 * t1846;
    let t20286 = t3237 * t1851;
    let t20290 = t1008 * t5561;
    let t20298 = 0.51448821741683684367e-2 * t15348 - 0.56688979511669985553e-2 * t20280 - 0.34013387707001991332e-1 * t15350 + 0.25724410870841842183e-2 * t15362 + 0.34299214494455789578e-2 * t15366 + 0.17149607247227894789e-2 * t15370 - 0.32012600194825403606e-1 * t20286 + 0.11337795902333997111e-1 * t12458 - 0.56688979511669985553e-2 * t12460 + 0.17149607247227894789e-1 * t20290 + 0.85748036236139473944e-2 * t418 * t1426 * t368 * t1713 * t1077 - 35.0 / 108.0 * t15378;
    (t20298,)
}
