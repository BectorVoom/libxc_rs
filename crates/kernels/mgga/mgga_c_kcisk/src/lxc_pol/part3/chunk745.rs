//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 745/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk745<F: Float>(t1814: F, t3293: F, t1824: F, t4684: F, t5093: F, t10593: F, t1842: F, t1856: F, t10585: F, t1659: F, t3845: F, t429: F, t686: F) -> (F, F, F, F, F, F) {
    let t11506 = t1814 * t3293;
    let t11507 = t11506 * t1824;
    let t11510 = t5093 * t4684;
    let t11513 = t1842 * t10593;
    let t11516 = t1856 * t10593;
    let t11519 = t1659 * t10585;
    let t11524 = F::new(0.27323333333333333333e-1) * t429 * t3845 * t686;
    (t11507, t11510, t11513, t11516, t11519, t11524)
}
