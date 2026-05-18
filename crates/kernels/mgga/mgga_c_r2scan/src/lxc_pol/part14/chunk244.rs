//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 244/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk244<F: Float>(t234: F, t750: F, t159: F, t216: F, t236: F, t41: F, t424: F, t594: F, t596: F, t598: F, t608: F, t619: F, t624: F, t629: F, t635: F, t647: F, t656: F, t658: F, t660: F, t661: F, t727: F, t731: F, t732: F, t738: F, t743: F, t747: F) -> (F, F) {
    let t752 = F::new(0.17315859105681463759e2) * t234 * t750;
    let t753 = -t594 - F::new(0.675260332e-1) * t596 * t598 + F::new(0.285764e-1) * t159 * t608 + t619 - t624 + t629 + t635 - t647 - t656 - t658 + t660 - t41 * t661 - t727 - F::new(0.21973736767207854065e-2) * t424 * t216 + t731 + F::new(0.5848223622634646207e0) * t732 * t236 - t738 - t743 + t747 + t752;
    (t752, t753)
}
