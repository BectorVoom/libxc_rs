//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 636/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk636(t174: f64, t5784: f64, t386: f64, t428: f64, t388: f64, t5679: f64, t384: f64, t418: f64, t4745: f64, t4747: f64, t4748: f64, t4750: f64, t4785: f64, t4843: f64, t4846: f64, t4881: f64, t4884: f64, t4889: f64, t4891: f64, t4897: f64, t6098: f64, t6102: f64, t6106: f64, t6110: f64, t6113: f64, t6116: f64) -> (f64, f64, f64, f64) {
    let t6119 = t174 * t5784;
    let t6121 = t386 * t428 * t6119;
    let t6125 = t386 * t5679 * t388;
    let t6126 = t384 * t6125;
    let t6133 = t4745 - t4747 + 0.25724410870841842183e-2_f64 * t4748 + 0.17149607247227894789e-2_f64 * t4750 - 0.42874018118069736972e-3_f64 * t6098 - 0.42874018118069736972e-3_f64 * t418 * t6102 + 0.85748036236139473944e-3_f64 * t418 * t6106 - 0.85748036236139473944e-3_f64 * t418 * t6110 + 0.42874018118069736972e-3_f64 * t6113 + 0.42874018118069736972e-3_f64 * t418 * t6116 + 0.42874018118069736972e-3_f64 * t418 * t6121 - 0.42874018118069736972e-3_f64 * t6126 + t4785 + 0.16006300097412701803e-1_f64 * t4843 + t4846 - 0.17149607247227894789e-2_f64 * t4881 - t4884 - 0.45351183609335988442e-1_f64 * t4889 - 0.22675591804667994221e-1_f64 * t4891 + 0.22675591804667994221e-1_f64 * t4897;
    (t6119, t6121, t6125, t6133)
}
