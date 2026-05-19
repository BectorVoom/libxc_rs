//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 294/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk294<F: Float>(t159: F, t216: F, t236: F, t41: F, t594: F, t598: F, t619: F, t624: F, t629: F, t635: F, t647: F, t656: F, t658: F, t660: F, t727: F, t731: F, t738: F, t743: F, t747: F, t752: F, t898: F, t951: F, t956: F, t959: F, t963: F) -> F {
    let t966 = -t594 - F::cast_from(0.675260332e-1_f64) * t951 * t598 + F::new(0.285764e-1) * t159 * t956 + t619 - t624 + t629 + t635 - t647 - t656 + t658 + t660 - t41 * t959 - t727 - F::cast_from(0.21973736767207854065e-2_f64) * t898 * t216 + t731 + F::cast_from(0.5848223622634646207e0_f64) * t963 * t236 - t738 - t743 + t747 + t752;
    t966
}
