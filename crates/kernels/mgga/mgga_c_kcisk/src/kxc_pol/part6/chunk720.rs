//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 720/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk720<F: Float>(t1646: F, t8533: F, t3521: F, t8896: F, t8908: F, t8912: F, t8920: F, t827: F, t8564: F, t8567: F, t8570: F, t45: F, t8584: F, t4787: F, t8607: F, t4744: F, t8573: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t22599 = t1646 * t8533;
    let t22646 = t3521 * t8896;
    let t22652 = t3521 * t8908;
    let t22654 = t3521 * t8912;
    let t22656 = t3521 * t8920;
    let t22698 = t827 * t8564;
    let t22705 = t827 * t8567;
    let t22707 = t827 * t8570;
    let t22750 = t45 * t8584;
    let t22760 = t4787 * t8607;
    let t22801 = t8573 * t4744;
    (t22599, t22646, t22652, t22654, t22656, t22698, t22705, t22707, t22750, t22760, t22801)
}
