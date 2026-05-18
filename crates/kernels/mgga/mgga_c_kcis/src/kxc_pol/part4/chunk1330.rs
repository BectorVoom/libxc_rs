//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1330/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1330<F: Float>(t11939: F, t1961: F, t3815: F, t1409: F, t4023: F, t1441: F, t1650: F, t11951: F, t12048: F, t167: F, t1444: F, t2622: F) -> (F, F, F, F, F, F) {
    let t17082 = t11939 * t1961;
    let t17083 = t17082 * t3815;
    let t17088 = t4023 * t1409;
    let t17096 = t1441 * t1650;
    let t17098 = t11951 * t1650;
    let t17100 = t12048 * t167;
    let t17102 = t2622 * t1444;
    (t17083, t17088, t17096, t17098, t17100, t17102)
}
