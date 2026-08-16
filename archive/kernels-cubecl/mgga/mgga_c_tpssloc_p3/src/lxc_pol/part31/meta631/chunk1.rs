//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1892/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1892<F: Float>(t22765: F, t6417: F, t6390: F, t80997: F, t81000: F, t1351: F, t3788: F, t6388: F, t6936: F, t19958: F, t22833: F, t22797: F, t6375: F) -> (F, F, F, F, F, F) {
    let t97378 = t22765 * t6417;
    let t97380 = t80997 * t6390;
    let t97382 = t81000 * t6390;
    let t97387 = t6936 * t3788 * t6388 * t1351;
    let t97389 = t22833 * t19958;
    let t97394 = t22797 * t6375;
    (t97378, t97380, t97382, t97387, t97389, t97394)
}
