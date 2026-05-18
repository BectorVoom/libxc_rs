//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 642/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk642<F: Float>(t2364: F, t2487: F, t4609: F, t1876: F, t4614: F, t7715: F, t1877: F, t7718: F, t4623: F, t8504: F, t706: F, t7034: F) -> (F, F, F, F, F, F) {
    let t8904 = t4609 * t2364 * t2487;
    let t8908 = t1876 * t4614 * t7715;
    let t8912 = t1876 * t1877 * t7718;
    let t8915 = t4623 * t8504;
    let t8916 = t706 * t8915;
    let t8919 = t7034 * t2487;
    (t8904, t8908, t8912, t8915, t8916, t8919)
}
