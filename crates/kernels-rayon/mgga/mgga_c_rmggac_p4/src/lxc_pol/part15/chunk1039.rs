//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1039/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1039(t17859: f64, t9198: f64, t2286: f64, t38472: f64, t1734: f64, t236: f64, t3352: f64, t495: f64, t7230: f64, t2320: f64, t38621: f64, t1364: f64, t1923: f64, t35772: f64, t35777: f64, t35782: f64, t35787: f64, t4041: f64, t40516: f64, t40559: f64, t40561: f64, t47048: f64, t47054: f64, t47062: f64, t534: f64, t6400: f64, t665: f64, t72: f64, t7894: f64, t9030: f64, t9954: f64) -> f64 {
    let t47071 = t17859 * t9198;
    let t47073 = t38472 * t2286;
    let t47078 = t7230 * t3352 * t236 * t1734 * t495;
    let t47081 = t38621 * t2320;
    let t47083 = 0.31923449919973379548e-4_f64 * t47048 - 0.51077519871957407276e-4_f64 * t47054 + 2.0_f64 * t72 * t534 * t9030 - 0.47896966807455234255e0_f64 * t40516 + 0.25538759935978703639e-4_f64 * t47062 - 0.2363e1_f64 * t1923 * t7894 - 0.47896966807455234256e0_f64 * t1364 * t665 * t6400 - 0.23948483403727617128e0_f64 * t4041 * t9954 - 0.25538759935978703638e-4_f64 * t47071 + 0.25538759935978703638e-4_f64 * t47073 - 0.15961724959986689774e-4_f64 * t47078 - 0.15243824895787514157e-3_f64 * t35772 - t35777 - t35782 + t35787 + 0.24829349937757072983e-4_f64 * t47081 - t40559 + t40561;
    t47083
}
