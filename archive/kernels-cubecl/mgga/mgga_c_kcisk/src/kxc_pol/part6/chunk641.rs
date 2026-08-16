//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 641/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk641<F: Float>(t1899: F, t8786: F, t1800: F, t1869: F, t2528: F, t6974: F, t2537: F, t6719: F, t4595: F, t4598: F, t7715: F, t2364: F, t2372: F, t4604: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8882 = t1899 * t8786;
    let t8883 = t1800 * t8882;
    let t8884 = t1869 * t8883;
    let t8886 = t6974 * t2528;
    let t8887 = t1869 * t8886;
    let t8889 = t6719 * t2537;
    let t8890 = t1869 * t8889;
    let t8896 = t4595 * t4598 * t7715;
    let t8900 = t4604 * t2364 * t2372;
    (t8882, t8883, t8884, t8886, t8887, t8889, t8890, t8896, t8900)
}
