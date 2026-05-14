//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 688/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk688<F: Float>(t10442: F, t1842: F, t5117: F, t970: F, t1856: F, t10585: F, t1835: F, t706: F, t10593: F, t1857: F, t3123: F, t5144: F, t5147: F, t10450: F, t10464: F, t5152: F, t960: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11545 = t1842 * t10442;
    let t11548 = t970 * t5117;
    let t11550 = t1856 * t10442;
    let t11553 = t1835 * t10585;
    let t11556 = t706 * t10585;
    let t11559 = t1835 * t10593;
    let t11562 = t3123 * t1857;
    let t11564 = t970 * t5144;
    let t11566 = t970 * t5147;
    let t11568 = t1856 * t10450;
    let t11571 = t706 * t10464;
    let t11574 = t960 * t5152;
    (t11545, t11548, t11550, t11553, t11556, t11559, t11562, t11564, t11566, t11568, t11571, t11574)
}
