//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1256/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1256<F: Float>(t22096: F, t3936: F, t5674: F, t13848: F, t6869: F, t9818: F, t9816: F, t13798: F, t13801: F, t13810: F, t13813: F, t22069: F, t22076: F, t22081: F, t22085: F, t22089: F, t22093: F, t3934: F, t5671: F) -> (F,) {
    let t22098 = t3936 * t5674 * t22096;
    let t22102 = t9818 * t13848 * t6869;
    let t22103 = t9816 * t22102;
    let t22105 = 0.25410001404642664113e-3 * t22069 - 35.0 / 108.0 * t13798 + 0.2032800112371413129e-4 * t13801 - 0.80031500487063509016e-2 * t13810 + t13813 + 0.85748036236139473944e-3 * t3934 * t22076 + 0.85748036236139473944e-3 * t3934 * t22081 - 0.21437009059034868486e-3 * t3934 * t22085 + 0.85748036236139473944e-3 * t5671 * t22089 + 0.17149607247227894789e-2 * t3934 * t22093 + 0.17149607247227894789e-2 * t3934 * t22098 + 0.10164000561857065645e-3 * t22103;
    (t22105,)
}
