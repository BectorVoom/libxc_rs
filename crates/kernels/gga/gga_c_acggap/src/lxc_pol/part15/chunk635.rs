//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 635/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk635<F: Float>(t174: F, t5784: F, t386: F, t428: F, t388: F, t5679: F, t384: F, t418: F, t4745: F, t4747: F, t4748: F, t4750: F, t4785: F, t4843: F, t4846: F, t4881: F, t4884: F, t4889: F, t4891: F, t4897: F, t6098: F, t6102: F, t6106: F, t6110: F, t6113: F, t6116: F) -> (F, F, F, F) {
    let t6119 = t174 * t5784;
    let t6121 = t386 * t428 * t6119;
    let t6125 = t386 * t5679 * t388;
    let t6126 = t384 * t6125;
    let t6133 = t4745 - t4747 + F::new(0.25724410870841842183e-2) * t4748 + F::new(0.17149607247227894789e-2) * t4750 - F::new(0.42874018118069736972e-3) * t6098 - F::new(0.42874018118069736972e-3) * t418 * t6102 + F::new(0.85748036236139473944e-3) * t418 * t6106 - F::new(0.85748036236139473944e-3) * t418 * t6110 + F::new(0.42874018118069736972e-3) * t6113 + F::new(0.42874018118069736972e-3) * t418 * t6116 + F::new(0.42874018118069736972e-3) * t418 * t6121 - F::new(0.42874018118069736972e-3) * t6126 + t4785 + F::new(0.16006300097412701803e-1) * t4843 + t4846 - F::new(0.17149607247227894789e-2) * t4881 - t4884 - F::new(0.45351183609335988442e-1) * t4889 - F::new(0.22675591804667994221e-1) * t4891 + F::new(0.22675591804667994221e-1) * t4897;
    (t6119, t6121, t6125, t6133)
}
