//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 673/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk673<F: Float>(t1824: F, t4640: F, t11285: F, t10585: F, t7000: F, t1648: F, t4648: F, t4604: F, t1636: F, t4652: F, t4609: F, t4684: F, t10593: F, t7012: F, t139: F, t3516: F, t41: F) -> (F, F, F, F, F, F, F, F) {
    let t11286 = t4640 * t1824;
    let t11287 = t11285 * t11286;
    let t11290 = t7000 * t10585;
    let t11293 = t4648 * t1648;
    let t11294 = t4604 * t11293;
    let t11297 = t1636 * t4652;
    let t11298 = t4604 * t11297;
    let t11301 = t4648 * t1824;
    let t11302 = t4609 * t11301;
    let t11305 = t1636 * t4684;
    let t11306 = t4609 * t11305;
    let t11309 = t7012 * t10593;
    let t11313 = t139 * t3516 * t41;
    (t11287, t11290, t11294, t11298, t11302, t11306, t11309, t11313)
}
