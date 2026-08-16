//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 926/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk926<F: Float>(t1919: F, t2063: F, t24434: F, t28368: F, t5249: F, t7389: F, t7718: F, t1920: F, t28312: F, t11832: F, t5248: F, t17991: F, t7715: F) -> (F, F, F, F, F, F) {
    let t29441 = t1919 * t24434 * t2063;
    let t29445 = t1919 * t5249 * t28368;
    let t29449 = t1919 * t7389 * t7718;
    let t29453 = t1919 * t1920 * t28312;
    let t29462 = t5248 * t11832 * t28368;
    let t29466 = t1919 * t17991 * t7715;
    (t29441, t29445, t29449, t29453, t29462, t29466)
}
