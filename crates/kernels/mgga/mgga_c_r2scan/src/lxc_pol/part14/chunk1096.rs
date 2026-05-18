//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1096/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1096<F: Float>(t38054: F, t38068: F, t38130: F, t38143: F, t38164: F, t38175: F, t38189: F, t11450: F, t3270: F, t1115: F, t1563: F, t36967: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38617 = F::new(0.39552774754617995815e1) * t38054;
    let t38622 = F::new(0.19634394786159580877e0) * t38068;
    let t38646 = F::new(0.28914548798370980346e-4) * t38130;
    let t38649 = F::new(0.23159605016379617484e1) * t38143;
    let t38657 = F::new(0.51410067763503603055e-4) * t38164;
    let t38661 = F::new(0.34909953929791734801e0) * t38175;
    let t38666 = F::new(0.46160609703545424213e1) * t38189;
    let t38678 = t3270 * t11450;
    let t38688 = t36967 * t1115 * t1563;
    (t38617, t38622, t38646, t38649, t38657, t38661, t38666, t38678, t38688)
}
