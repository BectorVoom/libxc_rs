//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1496/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1496<F: Float>(t11277: F, t11916: F, t11246: F, t11251: F, t3172: F, t1025: F, t1028: F, t11659: F, t11811: F, t11994: F, t12026: F, t15963: F, t3092: F, t3164: F, t3208: F, t3224: F, t371: F, t372: F, t373: F, t42097: F, t42346: F, t42355: F, t42360: F, t42369: F, t42371: F, t4899: F) -> F {
    let t42374 = t11277 * t11916;
    let t42377 = t11246 * t3172 * t11251;
    let t42379 = F::cast_from(0.57165357490759649296e-3_f64) * t42346 - F::cast_from(0.85748036236139473944e-3_f64) * t3224 * t11811 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t371 * t372 * t373 * t42097 + F::cast_from(0.21240106161011140804e0_f64) * t42355 * t1028 + F::cast_from(0.25724410870841842184e-2_f64) * t42360 * t3208 + F::cast_from(0.17149607247227894789e-2_f64) * t4899 * t3092 * t11659 * t15963 - F::cast_from(0.17149607247227894789e-2_f64) * t11994 * t12026 + F::cast_from(0.34299214494455789578e-2_f64) * t42369 + F::cast_from(0.13719685797782315831e-1_f64) * t42371 * t3164 - F::cast_from(0.17149607247227894789e-2_f64) * t42374 - F::cast_from(0.34299214494455789578e-2_f64) * t42377;
    t42379
}
