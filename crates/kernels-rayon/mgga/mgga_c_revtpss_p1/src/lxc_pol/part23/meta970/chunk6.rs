//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3276/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3276(t22865: f64, t9918: f64, t1883: f64, t6883: f64, t9816: f64, t9818: f64, t1399: f64, t22046: f64, t22074: f64, t22096: f64, t3934: f64, t3936: f64, t48869: f64, t48872: f64, t48877: f64, t48879: f64, t5659: f64, t5673: f64, t74471: f64, t74475: f64, t74479: f64, t74481: f64, t74485: f64, t74489: f64, t74491: f64, t74493: f64, t74498: f64, t85609: f64) -> f64 {
    let t86112 = t9918 * t22865;
    let t86124 = t9816 * t9818 * t6883 * t1883;
    let t86136 = 0.25724410870841842183e-2_f64 * t3934 * t3936 * t22046 * t22096 - 0.12004725073059526352e-1_f64 * t74471 - 0.60023625365297631763e-2_f64 * t86112 - 0.85748036236139473944e-4_f64 * t74475 + 0.7623000421392799234e-3_f64 * t74479 - 0.6002362536529763176e-1_f64 * t74481 + 0.1084295579938911763e-3_f64 * t74485 + 0.25724410870841842183e-2_f64 * t3934 * t3936 * t22074 * t5659 + 0.15246000842785598467e-3_f64 * t86124 - 0.76230004213927992338e-3_f64 * t74489 - 0.21437009059034868486e-3_f64 * t3934 * t5673 * t85609 * t1399 - 0.91464571985215438872e-3_f64 * t74491 + 0.45732285992607719437e-2_f64 * t74493 - 0.12004725073059526352e-1_f64 * t74498 + t48869 + 0.27107389498472794075e-3_f64 * t48872 - t48877 + 0.24396650548625514667e-3_f64 * t48879;
    t86136
}
