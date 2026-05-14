//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1097/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1097<F: Float>(t2987: F, t555: F, t560: F, t1783: F, t6160: F, t1788: F, t10: F, t19706: F, t1897: F, t28: F, t1806: F, t1815: F, t1804: F, t1809: F, t1797: F, t6025: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20022 = t555 * t2987 * t560;
    let t20070 = t555 * t6160 * t1783;
    let t20073 = t555 * t6160 * t1788;
    let t20075 = t19706 * t10;
    let t20078 = 1.0 / t28 / t1897;
    let t20127 = t1815 * t1806;
    let t20129 = t1804 * t20127 * t1809;
    let t20132 = t555 * t6160 * t1797;
    let t20162 = t6025 * t10;
    (t20022, t20070, t20073, t20075, t20078, t20127, t20129, t20132, t20162)
}
