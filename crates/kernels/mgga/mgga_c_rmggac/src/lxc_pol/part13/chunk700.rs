//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 700/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk700<F: Float>(t1969: F, t35657: F, t1987: F, t34881: F, t4685: F, t511: F, t1982: F, t7428: F, t7434: F, t1326: F, t2016: F, t7551: F, t2049: F, t35253: F, t7760: F, t2019: F, t271: F, t3118: F, t641: F) -> (F, F, F, F, F, F, F) {
    let t35658 = t35657 * t1969;
    let t35665 = t34881 * t1987;
    let t35674 = t4685 * t511;
    let t35683 = t7434 * t7428 * t1982;
    let t35688 = t2016 * t7551 * t1326;
    let t35691 = t35688 * t2049 * t35253 * t7760;
    let t35696 = t2019 * t3118 * t271 * t641;
    (t35658, t35665, t35674, t35683, t35688, t35691, t35696)
}
