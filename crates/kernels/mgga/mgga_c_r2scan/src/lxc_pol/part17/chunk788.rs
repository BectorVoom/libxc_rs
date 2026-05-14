//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 788/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk788<F: Float>(t246: F, t4721: F, t4901: F, t4964: F, t4967: F, t4972: F, t7861: F, t8552: F, t8555: F, t8556: F, t9005: F, t9040: F, t4791: F, t4794: F, t4798: F, t4975: F, t4979: F, t4981: F, t6961: F, t7865: F, t8559: F, t8560: F, t8592: F) -> (F, F) {
    let t9044 = t8552 - t4901 + t8555 + t7861 + 0.285764e-1 * t9040 - 0.285764e-1 * t246 * t9005 - t4721 + t4964 - t4967 - t8556 - t4972;
    let t9047 = t4975 - t8559 - t8560 + t4979 - t4981 - t6961 + 0.571528e-1 * t7865 - t8592 - t4791 + t4794 + t4798;
    (t9044, t9047)
}
