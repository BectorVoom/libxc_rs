//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 666/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk666<F: Float>(t645: F, t10975: F, t11136: F, t67: F, t1848: F, t641: F, t916: F, t1757: F, t4972: F, t10522: F, t1755: F, t10436: F, t1751: F, t1758: F, t340: F, t4962: F, t4973: F, t4977: F, t6141: F, t639: F, t642: F, t7196: F) -> (F, F, F, F) {
    let t646 = t645 < -0.66725e-1;
    let t11138 = t67 * (t10975 + t11136);
    let t11153 = 1.0 / t641 / t916 / t1848;
    let t11154 = t4972 * t1757;
    let t11155 = t11153 * t11154;
    let t11162 = t1755 * t10522;
    let t11167 = piecewise3(t646, 0.0, 10.0 / 9.0 * t340 * t11138 * t642 - 10.0 / 9.0 * t340 * t4962 * t1758 + 40.0 / 27.0 * t340 * t1751 * t4973 - 10.0 / 9.0 * t340 * t1751 * t4977 - 280.0 / 243.0 * t340 * t639 * t11155 + 40.0 / 27.0 * t6141 * t7196 * t10436 - 10.0 / 27.0 * t340 * t639 * t11162);
    (t11154, t11155, t11162, t11167)
}
