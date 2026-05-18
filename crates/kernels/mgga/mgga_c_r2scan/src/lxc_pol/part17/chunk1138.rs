//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1138/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1138<F: Float>(t39939: F, t39941: F, t39958: F, t39962: F, t39964: F, t39967: F, t39969: F, t39979: F, t39984: F, t39995: F, t40001: F, t40041: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41623 = F::new(0.21951497276451705328e-1) * t39939;
    let t41624 = F::new(0.46230515946956099004e0) * t39941;
    let t41634 = F::new(0.18629842481251516498e0) * t39958;
    let t41635 = F::new(0.28565981518604370584e-1) * t39962;
    let t41636 = F::new(0.95219938395347901946e-2) * t39964;
    let t41637 = F::new(0.95219938395347901946e-2) * t39967;
    let t41638 = F::new(0.28565981518604370584e-1) * t39969;
    let t41642 = F::new(0.21951497276451705328e-1) * t39979;
    let t41644 = F::new(0.10975748638225852664e-1) * t39984;
    let t41649 = F::new(0.27944763721877274748e0) * t39995;
    let t41651 = F::new(0.27944763721877274748e0) * t40001;
    let t41668 = F::new(0.93149212406257582492e-1) * t40041;
    (t41623, t41624, t41634, t41635, t41636, t41637, t41638, t41642, t41644, t41649, t41651, t41668)
}
