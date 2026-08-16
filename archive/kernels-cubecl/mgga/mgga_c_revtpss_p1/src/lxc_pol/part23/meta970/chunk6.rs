//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3276/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3276<F: Float>(t22865: F, t9918: F, t1883: F, t6883: F, t9816: F, t9818: F, t1399: F, t22046: F, t22074: F, t22096: F, t3934: F, t3936: F, t48869: F, t48872: F, t48877: F, t48879: F, t5659: F, t5673: F, t74471: F, t74475: F, t74479: F, t74481: F, t74485: F, t74489: F, t74491: F, t74493: F, t74498: F, t85609: F) -> F {
    let t86112 = t9918 * t22865;
    let t86124 = t9816 * t9818 * t6883 * t1883;
    let t86136 = F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t22046 * t22096 - F::cast_from(0.12004725073059526352e-1_f64) * t74471 - F::cast_from(0.60023625365297631763e-2_f64) * t86112 - F::cast_from(0.85748036236139473944e-4_f64) * t74475 + F::cast_from(0.7623000421392799234e-3_f64) * t74479 - F::cast_from(0.6002362536529763176e-1_f64) * t74481 + F::cast_from(0.1084295579938911763e-3_f64) * t74485 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t3936 * t22074 * t5659 + F::cast_from(0.15246000842785598467e-3_f64) * t86124 - F::cast_from(0.76230004213927992338e-3_f64) * t74489 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t5673 * t85609 * t1399 - F::cast_from(0.91464571985215438872e-3_f64) * t74491 + F::cast_from(0.45732285992607719437e-2_f64) * t74493 - F::cast_from(0.12004725073059526352e-1_f64) * t74498 + t48869 + F::cast_from(0.27107389498472794075e-3_f64) * t48872 - t48877 + F::cast_from(0.24396650548625514667e-3_f64) * t48879;
    t86136
}
