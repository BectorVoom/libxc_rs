//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1726/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1726<F: Float>(t1412: F, t2661: F, t3938: F, t3992: F, t4056: F, t9810: F, t9979: F, t124: F, t1388: F, t1390: F, t3934: F, t3944: F, t4002: F, t46298: F, t46547: F, t46574: F, t46628: F, t46719: F, t46723: F, t46730: F, t46741: F, t46747: F, t46749: F, t46754: F, t46757: F, t46760: F, t46767: F, t46771: F, t800: F, t828: F, t9826: F, t9955: F, t9956: F) -> F {
    let t46776 = t2661 * t3992 * t1412 * t4056 * t3938;
    let t46780 = t2661 * t3992 * t9979 * t9810;
    let t46782 = -F::cast_from(0.6098400337114239387e-2_f64) * t46719 + F::cast_from(0.15117061203111996148e0_f64) * t46723 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t1390 * t828 * t46574 + F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t46730 * t800 * t124 * t46628 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3944 * t800 * t124 * t46298 - F::cast_from(0.65049603595885220128e-2_f64) * t46741 - F::cast_from(0.25724410870841842184e-1_f64) * t3934 * t9955 * t9826 * t9956 - F::cast_from(0.24009450146119052704e-1_f64) * t46747 - F::cast_from(0.48018900292238105408e-1_f64) * t46749 + F::cast_from(0.60984003371142393869e-3_f64) * t46754 - F::cast_from(0.27107389498472794074e-4_f64) * t46757 - t46760 + F::cast_from(0.30011812682648815881e-2_f64) * t4002 * t1390 * t828 * t46547 + F::cast_from(0.68026775414003982664e-1_f64) * t46767 + F::cast_from(0.85748036236139473944e-4_f64) * t46771 - F::cast_from(0.34299214494455789577e-3_f64) * t46776 - F::cast_from(0.34299214494455789577e-3_f64) * t46780;
    t46782
}
