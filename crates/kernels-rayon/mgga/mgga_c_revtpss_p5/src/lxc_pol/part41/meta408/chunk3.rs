//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1430/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1430(t13944: f64, t3936: f64, t6869: f64, t543: f64, t5591: f64, t5674: f64, t13848: f64, t9818: f64, t9816: f64, t13798: f64, t13801: f64, t13810: f64, t13813: f64, t22069: f64, t22076: f64, t22081: f64, t22085: f64, t22089: f64, t3934: f64, t5671: f64) -> f64 {
    let t22093 = t3936 * t13944 * t6869;
    let t22096 = t543 * t5591;
    let t22098 = t3936 * t5674 * t22096;
    let t22102 = t9818 * t13848 * t6869;
    let t22103 = t9816 * t22102;
    let t22105 = 0.25410001404642664113e-3_f64 * t22069 - 35.0_f64 / 108.0_f64 * t13798 + 0.2032800112371413129e-4_f64 * t13801 - 0.80031500487063509016e-2_f64 * t13810 + t13813 + 0.85748036236139473944e-3_f64 * t3934 * t22076 + 0.85748036236139473944e-3_f64 * t3934 * t22081 - 0.21437009059034868486e-3_f64 * t3934 * t22085 + 0.85748036236139473944e-3_f64 * t5671 * t22089 + 0.17149607247227894789e-2_f64 * t3934 * t22093 + 0.17149607247227894789e-2_f64 * t3934 * t22098 + 0.10164000561857065645e-3_f64 * t22103;
    t22105
}
