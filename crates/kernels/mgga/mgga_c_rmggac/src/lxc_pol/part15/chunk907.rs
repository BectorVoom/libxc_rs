//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 907/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk907<F: Float>(t2286: F, t38472: F, t1734: F, t236: F, t3352: F, t495: F, t7230: F, t2320: F, t38621: F, t1364: F, t1923: F, t35772: F, t35777: F, t35782: F, t35787: F, t4041: F, t40516: F, t40559: F, t40561: F, t47048: F, t47054: F, t47062: F, t47071: F, t534: F, t6400: F, t665: F, t72: F, t7894: F, t9030: F, t9954: F) -> (F,) {
    let t47073 = t38472 * t2286;
    let t47078 = t7230 * t3352 * t236 * t1734 * t495;
    let t47081 = t38621 * t2320;
    let t47083 = 0.31923449919973379548e-4 * t47048 - 0.51077519871957407276e-4 * t47054 + 2.0 * t72 * t534 * t9030 - 0.47896966807455234255e0 * t40516 + 0.25538759935978703639e-4 * t47062 - 0.2363e1 * t1923 * t7894 - 0.47896966807455234256e0 * t1364 * t665 * t6400 - 0.23948483403727617128e0 * t4041 * t9954 - 0.25538759935978703638e-4 * t47071 + 0.25538759935978703638e-4 * t47073 - 0.15961724959986689774e-4 * t47078 - 0.15243824895787514157e-3 * t35772 - t35777 - t35782 + t35787 + 0.24829349937757072983e-4 * t47081 - t40559 + t40561;
    (t47083,)
}
