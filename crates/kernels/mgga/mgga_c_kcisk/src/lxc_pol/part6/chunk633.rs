//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 633/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk633<F: Float>(t143: F, t4597: F, t3845: F, t429: F, t686: F, t5814: F, t79: F, t435: F, t690: F, t41: F, t5821: F, t698: F, t445: F, t1849: F, t213: F, t967: F) -> (F, F, F, F, F, F, F, F) {
    let t11495 = t143 * t4597;
    let t11524 = 0.27323333333333333333e-1 * t429 * t3845 * t686;
    let t11525 = t5814 * t79;
    let t11528 = 0.77488888888888888888e-2 * t435 * t11525 * t690;
    let t11529 = t5821 * t41;
    let t11530 = t11529 * t698;
    let t11532 = 0.72818958333333333333e-4 * t445 * t11530;
    let t11612 = t213 * t1849;
    let t11625 = t967 * t4597;
    (t11495, t11524, t11525, t11528, t11529, t11532, t11612, t11625)
}
