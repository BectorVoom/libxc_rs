//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 984/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk984<F: Float>(t2231: F, t3786: F, t1341: F, t13330: F, t1411: F, t3764: F, t5991: F, t3785: F, t3739: F, t6008: F, t2152: F, t3732: F, t1450: F, t1415: F, t3495: F, t5606: F) -> (F, F, F, F, F, F, F) {
    let t19917 = t2231 * t3786;
    let t19918 = t1341 * t19917;
    let t19919 = t13330 * t19918;
    let t19920 = t1411 * t19919;
    let t19922 = t3764 * t5991;
    let t19923 = t3785 * t19922;
    let t19924 = t1411 * t19923;
    let t19926 = t3739 * t6008;
    let t19928 = t2152 * t3732;
    let t19929 = t1450 * t19928;
    let t19930 = t1415 * t19929;
    let t19931 = t1411 * t19930;
    let t19935 = t5606 * t3495;
    (t19917, t19920, t19924, t19926, t19928, t19931, t19935)
}
