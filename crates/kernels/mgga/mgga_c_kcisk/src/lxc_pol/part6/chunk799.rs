//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 799/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk799<F: Float>(t2487: F, t4609: F, t8514: F, t11269: F, t2364: F, t8504: F, t2372: F, t4604: F, t28385: F, t7012: F, t8518: F, t22927: F, t4629: F, t7034: F, t8536: F, t16892: F, t8500: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28826 = t4609 * t8514 * t2487;
    let t28830 = t11269 * t2364 * t8504;
    let t28834 = t4604 * t8514 * t2372;
    let t28837 = t7012 * t28385;
    let t28841 = t4604 * t8518 * t2372;
    let t28846 = t22927 * t2487;
    let t28847 = t4629 * t28846;
    let t28851 = t7034 * t8536;
    let t28852 = t4629 * t28851;
    let t28855 = t16892 * t8500;
    (t28826, t28830, t28834, t28837, t28841, t28846, t28847, t28851, t28852, t28855)
}
