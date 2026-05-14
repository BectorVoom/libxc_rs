//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 719/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk719<F: Float>(t1869: F, t8886: F, t2537: F, t6719: F, t4595: F, t4598: F, t7715: F, t2364: F, t2372: F, t4604: F, t2487: F, t4609: F, t1876: F, t4614: F, t1877: F, t7718: F) -> (F, F, F, F, F, F, F, F) {
    let t8887 = t1869 * t8886;
    let t8889 = t6719 * t2537;
    let t8890 = t1869 * t8889;
    let t8896 = t4595 * t4598 * t7715;
    let t8900 = t4604 * t2364 * t2372;
    let t8904 = t4609 * t2364 * t2487;
    let t8908 = t1876 * t4614 * t7715;
    let t8912 = t1876 * t1877 * t7718;
    (t8887, t8889, t8890, t8896, t8900, t8904, t8908, t8912)
}
