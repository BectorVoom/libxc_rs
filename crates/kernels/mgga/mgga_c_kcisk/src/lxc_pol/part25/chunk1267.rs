//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1267/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1267<F: Float>(t1310: F, t1774: F, t5507: F, t112772: F, t33177: F, t33162: F, t33258: F, t11966: F, t2020: F, t123: F, t2801: F, t33282: F, t10879: F, t9740: F, t9742: F, t48397: F, t79: F) -> (F, F, F, F, F, F, F) {
    let t112876 = t1310 * t1774 * t5507;
    let t112881 = t33177 * t112772;
    let t112889 = t33258 * t33162;
    let t112904 = t2020 * t11966;
    let t112921 = t2801 * t33282 * t123;
    let t112925 = t9740 * t10879 * t9742;
    let t112933 = t48397 * t79;
    (t112876, t112881, t112889, t112904, t112921, t112925, t112933)
}
