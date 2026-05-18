//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 888/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk888<F: Float>(t1799: F, t28817: F, t11259: F, t2364: F, t8500: F, t2487: F, t4609: F, t8514: F, t11269: F, t8504: F, t2372: F, t4604: F) -> (F, F, F, F, F) {
    let t28818 = t1799 * t28817;
    let t28822 = t11259 * t2364 * t8500;
    let t28826 = t4609 * t8514 * t2487;
    let t28830 = t11269 * t2364 * t8504;
    let t28834 = t4604 * t8514 * t2372;
    (t28818, t28822, t28826, t28830, t28834)
}
