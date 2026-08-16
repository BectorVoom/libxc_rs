//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1496/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1496(t11277: f64, t11916: f64, t11246: f64, t11251: f64, t3172: f64, t1025: f64, t1028: f64, t11659: f64, t11811: f64, t11994: f64, t12026: f64, t15963: f64, t3092: f64, t3164: f64, t3208: f64, t3224: f64, t371: f64, t372: f64, t373: f64, t42097: f64, t42346: f64, t42355: f64, t42360: f64, t42369: f64, t42371: f64, t4899: f64) -> f64 {
    let t42374 = t11277 * t11916;
    let t42377 = t11246 * t3172 * t11251;
    let t42379 = 0.57165357490759649296e-3_f64 * t42346 - 0.85748036236139473944e-3_f64 * t3224 * t11811 - 0.21437009059034868486e-3_f64 * t1025 * t371 * t372 * t373 * t42097 + 0.21240106161011140804e0_f64 * t42355 * t1028 + 0.25724410870841842184e-2_f64 * t42360 * t3208 + 0.17149607247227894789e-2_f64 * t4899 * t3092 * t11659 * t15963 - 0.17149607247227894789e-2_f64 * t11994 * t12026 + 0.34299214494455789578e-2_f64 * t42369 + 0.13719685797782315831e-1_f64 * t42371 * t3164 - 0.17149607247227894789e-2_f64 * t42374 - 0.34299214494455789578e-2_f64 * t42377;
    t42379
}
