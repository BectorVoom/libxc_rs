//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 244/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk244(t234: f64, t750: f64, t159: f64, t216: f64, t236: f64, t41: f64, t424: f64, t594: f64, t596: f64, t598: f64, t608: f64, t619: f64, t624: f64, t629: f64, t635: f64, t647: f64, t656: f64, t658: f64, t660: f64, t661: f64, t727: f64, t731: f64, t732: f64, t738: f64, t743: f64, t747: f64) -> (f64, f64) {
    let t752 = 0.17315859105681463759e2_f64 * t234 * t750;
    let t753 = -t594 - 0.675260332e-1_f64 * t596 * t598 + 0.285764e-1_f64 * t159 * t608 + t619 - t624 + t629 + t635 - t647 - t656 - t658 + t660 - t41 * t661 - t727 - 0.21973736767207854065e-2_f64 * t424 * t216 + t731 + 0.5848223622634646207e0_f64 * t732 * t236 - t738 - t743 + t747 + t752;
    (t752, t753)
}
