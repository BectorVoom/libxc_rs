//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 998/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk998<F: Float>(t645: F, t17561: F, t17675: F, t67: F, t1750: F, t4971: F, t11153: F, t638: F, t16617: F, t17132: F, t1758: F, t2436: F, t2442: F, t340: F, t4962: F, t4973: F, t4977: F, t6141: F, t642: F, t6707: F, t7186: F, t7196: F) -> (F,) {
    let t646 = t645 < -0.66725e-1;
    let t17677 = t67 * (t17561 + t17675);
    let t17693 = t1750 * t4971;
    let t17697 = t638 * t11153;
    let t17705 = piecewise3(t646, 0.0, 10.0 / 9.0 * t340 * t17677 * t642 - 20.0 / 27.0 * t340 * t7186 * t1758 + 40.0 / 81.0 * t340 * t2436 * t4973 - 10.0 / 27.0 * t340 * t2436 * t4977 - 10.0 / 27.0 * t340 * t4962 * t2442 + 80.0 / 81.0 * t6141 * t17693 * t6707 - 280.0 / 243.0 * t6141 * t17697 * t17132 + 40.0 / 81.0 * t6141 * t7196 * t16617);
    (t17705,)
}
