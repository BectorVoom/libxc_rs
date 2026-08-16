//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1726/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1726(t1412: f64, t2661: f64, t3938: f64, t3992: f64, t4056: f64, t9810: f64, t9979: f64, t124: f64, t1388: f64, t1390: f64, t3934: f64, t3944: f64, t4002: f64, t46298: f64, t46547: f64, t46574: f64, t46628: f64, t46719: f64, t46723: f64, t46730: f64, t46741: f64, t46747: f64, t46749: f64, t46754: f64, t46757: f64, t46760: f64, t46767: f64, t46771: f64, t800: f64, t828: f64, t9826: f64, t9955: f64, t9956: f64) -> f64 {
    let t46776 = t2661 * t3992 * t1412 * t4056 * t3938;
    let t46780 = t2661 * t3992 * t9979 * t9810;
    let t46782 = -0.6098400337114239387e-2_f64 * t46719 + 0.15117061203111996148e0_f64 * t46723 - 0.21437009059034868486e-3_f64 * t1388 * t1390 * t828 * t46574 + 5.0_f64 / 4.0_f64 * t46730 * t800 * t124 * t46628 + 3.0_f64 / 16.0_f64 * t3944 * t800 * t124 * t46298 - 0.65049603595885220128e-2_f64 * t46741 - 0.25724410870841842184e-1_f64 * t3934 * t9955 * t9826 * t9956 - 0.24009450146119052704e-1_f64 * t46747 - 0.48018900292238105408e-1_f64 * t46749 + 0.60984003371142393869e-3_f64 * t46754 - 0.27107389498472794074e-4_f64 * t46757 - t46760 + 0.30011812682648815881e-2_f64 * t4002 * t1390 * t828 * t46547 + 0.68026775414003982664e-1_f64 * t46767 + 0.85748036236139473944e-4_f64 * t46771 - 0.34299214494455789577e-3_f64 * t46776 - 0.34299214494455789577e-3_f64 * t46780;
    t46782
}
