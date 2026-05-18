//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 764/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk764<F: Float>(t1965: F, t7942: F, t1969: F, t1987: F, t34881: F, t4685: F, t511: F, t1982: F, t7428: F, t7434: F, t1326: F, t2016: F, t7551: F) -> (F, F, F, F, F) {
    let t35657 = t7942 * t1965;
    let t35658 = t35657 * t1969;
    let t35665 = t34881 * t1987;
    let t35674 = t4685 * t511;
    let t35683 = t7434 * t7428 * t1982;
    let t35688 = t2016 * t7551 * t1326;
    (t35658, t35665, t35674, t35683, t35688)
}
