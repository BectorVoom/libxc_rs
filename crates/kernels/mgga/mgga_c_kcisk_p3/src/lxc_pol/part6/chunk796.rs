//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 796/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk796<F: Float>(t4787: F, t8607: F, t4744: F, t8573: F, t1644: F, t8544: F, t682: F, t8522: F, t8504: F, t1417: F, t8928: F, t719: F, t8831: F) -> (F, F, F, F, F, F, F) {
    let t22760 = t4787 * t8607;
    let t22801 = t8573 * t4744;
    let t22891 = t8544 * t1644;
    let t22927 = t682 * t8522;
    let t22937 = t682 * t8504;
    let t22942 = t1417 * t8928;
    let t23033 = t8831 * t719;
    (t22760, t22801, t22891, t22927, t22937, t22942, t23033)
}
