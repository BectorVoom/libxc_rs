//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 943/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk943<F: Float>(t16662: F, t1869: F, t4817: F, t6690: F, t5074: F, t6694: F, t1797: F, t2507: F, t1336: F, t140: F, t5196: F, t6961: F, t4581: F, t6966: F, t4811: F, t6686: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16663 = t1869 * t16662;
    let t16669 = t4817 * t6690;
    let t16670 = t1869 * t16669;
    let t16672 = t5074 * t6694;
    let t16673 = 0.22109259259259259258e-2 * t16672;
    let t16674 = t1797 * t2507;
    let t16676 = t140 * t1336 * t16674;
    let t16677 = t16676 * t5196;
    let t16681 = t4817 * t6961;
    let t16682 = t1869 * t16681;
    let t16684 = t4581 * t6966;
    let t16685 = t1869 * t16684;
    let t16687 = t4811 * t6686;
    (t16663, t16670, t16672, t16673, t16676, t16677, t16682, t16685, t16687)
}
