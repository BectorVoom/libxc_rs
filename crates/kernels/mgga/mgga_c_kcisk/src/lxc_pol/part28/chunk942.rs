//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 942/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk942<F: Float>(t22337: F, t6677: F, t16676: F, t6669: F, t6663: F, t1802: F, t22254: F, t1799: F, t6684: F, t6697: F, t5062: F, t1869: F, t10494: F, t8959: F, t5074: F, t8955: F) -> (F, F, F, F, F, F, F) {
    let t22338 = t22337 * t6677;
    let t22340 = t16676 * t6669;
    let t22342 = t16676 * t6663;
    let t22346 = t22254 * t1802;
    let t22347 = t1799 * t22346;
    let t22349 = t6697 * t6684;
    let t22350 = t5062 * t22349;
    let t22351 = t1869 * t22350;
    let t22353 = t10494 * t8959;
    let t22355 = t5074 * t8955;
    (t22338, t22340, t22342, t22347, t22351, t22353, t22355)
}
