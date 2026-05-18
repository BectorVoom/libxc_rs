//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 982/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk982<F: Float>(t2847: F, t797: F, t2526: F, t2333: F, t983: F, t795: F, t2867: F, t792: F, t158: F, t955: F, t874: F, t3446: F, t3447: F) -> (F, F, F, F, F, F, F, F) {
    let t11531 = t797 * t2847;
    let t11550 = t797 * t2526;
    let t11554 = t2333 * t983;
    let t11555 = t11554 * t795;
    let t11559 = t2867 * t792;
    let t11563 = t158 * t955;
    let t11564 = t11563 * t874;
    let t11566 = t3446 * t3447 * t11564;
    (t11531, t11550, t11554, t11555, t11559, t11563, t11564, t11566)
}
