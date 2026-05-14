//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 727/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk727<F: Float>(t2321: F, t3973: F, t1580: F, t4391: F, t5626: F, t3952: F, t1591: F, t2059: F, t4400: F, t1312: F, t4406: F, t1581: F, t220: F, t6187: F, t2327: F, t4419: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6473 = t3973 * t2321;
    let t6474 = t1580 * t6473;
    let t6476 = t4391 * t5626;
    let t6477 = t3952 * t6476;
    let t6480 = t2059 * t1591;
    let t6481 = t4400 * t6480;
    let t6482 = t1312 * t6481;
    let t6485 = t4406 * t5626;
    let t6486 = t1312 * t6485;
    let t6489 = t1581 * t220;
    let t6490 = t6187 * t6489;
    let t6497 = t4419 * t2327;
    (t6473, t6474, t6476, t6477, t6480, t6481, t6482, t6485, t6486, t6489, t6490, t6497)
}
