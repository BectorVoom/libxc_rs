//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 869/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk869<F: Float>(t14961: F, t17749: F, t17753: F, t4199: F, t19263: F, t2771: F, t1775: F, t5346: F, t458: F, t5360: F, t5356: F, t19011: F, t10603: F, t19016: F, t19020: F, t17744: F, t4206: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19640 = t14961 * t17749;
    let t19643 = t4199 * t17753;
    let t19646 = t2771 * t19263;
    let t19649 = t1775 * t5346;
    let t19651 = t458 * t5360;
    let t19653 = t458 * t5356;
    let t19656 = t2771 * t19011;
    let t19659 = t10603 * t19016;
    let t19662 = t2771 * t19020;
    let t19665 = t4206 * t17744;
    (t19640, t19643, t19646, t19649, t19651, t19653, t19656, t19659, t19662, t19665)
}
