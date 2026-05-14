//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 914/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk914<F: Float>(t5606: F, t8093: F, t3759: F, t1341: F, t30298: F, t1340: F, t1339: F, t2231: F, t8010: F, t3776: F, t1415: F, t1411: F, t30290: F, t7710: F, t3797: F, t3796: F) -> (F, F, F, F, F, F) {
    let t30955 = t5606 * t8093;
    let t30956 = t3759 * t30955;
    let t30958 = t1341 * t30298;
    let t30959 = t1340 * t30958;
    let t30960 = t1339 * t30959;
    let t30962 = t2231 * t8010;
    let t30963 = t3776 * t30962;
    let t30964 = t1415 * t30963;
    let t30965 = t1411 * t30964;
    let t30967 = t1341 * t30290;
    let t30968 = t1340 * t30967;
    let t30969 = t3759 * t30968;
    let t30972 = t7710 * t2231;
    let t30973 = t3797 * t30972;
    let t30974 = t3796 * t30973;
    (t30956, t30960, t30962, t30965, t30969, t30974)
}
