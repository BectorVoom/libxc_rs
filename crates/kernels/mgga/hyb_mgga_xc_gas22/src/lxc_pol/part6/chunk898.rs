//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 898/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk898<F: Float>(t1252: F, t6270: F, t1248: F, t6012: F, t1890: F, t3174: F, t3180: F, t3184: F, t7942: F, t39: F, t6289: F, t1238: F, t6291: F, t2028: F, t6299: F, t3171: F) -> (F, F, F, F, F, F, F, F) {
    let t8267 = t6270 * t1252;
    let t8288 = t6012 * t1248;
    let t8291 = 2.0 / 243.0 * t1890 * t3174;
    let t8293 = 2.0 / 81.0 * t1890 * t3180;
    let t8294 = t7942 * t3184;
    let t8296 = t6289 * t39;
    let t8297 = t6291 * t1238;
    let t8299 = t8296 * t8297 * t2028;
    let t8302 = t6299 * t1238;
    let t8304 = t3171 * t8302 * t2028;
    (t8267, t8288, t8291, t8293, t8294, t8296, t8299, t8304)
}
