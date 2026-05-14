//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1048/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1048<F: Float>(t2002: F, t28524: F, t303: F, t1983: F, t2012: F, t7086: F, t7914: F, t6176: F, t15955: F, t2011: F, t27387: F, t1464: F, t6284: F, t7909: F, t5709: F, t27438: F, t6281: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29337 = t28524 * t2002;
    let t29338 = t303 * t29337;
    let t29340 = t1983 * t2012;
    let t29341 = t303 * t29340;
    let t29343 = t7914 * t7086;
    let t29344 = t6176 * t29343;
    let t29353 = t15955 * t2011;
    let t29354 = t27387 * t29353;
    let t29355 = t1464 * t29354;
    let t29357 = t7909 * t6284;
    let t29358 = t5709 * t29357;
    let t29361 = t27438 * t6281;
    (t29337, t29338, t29340, t29341, t29343, t29344, t29353, t29354, t29355, t29357, t29358, t29361)
}
