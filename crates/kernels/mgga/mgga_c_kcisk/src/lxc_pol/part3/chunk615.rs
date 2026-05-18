//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 615/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk615<F: Float>(t5298: F, t735: F, t734: F, t4803: F, t642: F, t5174: F, t716: F, t740: F, t748: F, t4816: F, t1950: F, t1945: F, t1954: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5299 = t735 * t5298;
    let t5300 = t734 * t5299;
    let t5302 = t642 * t4803;
    let t5303 = t735 * t5302;
    let t5304 = t734 * t5303;
    let t5306 = t5174 * t716;
    let t5307 = t5306 * t740;
    let t5308 = t5307 * t748;
    let t5310 = t4816 * t740;
    let t5311 = t5310 * t1950;
    let t5313 = t1945 * t1954;
    (t5299, t5300, t5302, t5303, t5304, t5306, t5307, t5308, t5310, t5311, t5313)
}
