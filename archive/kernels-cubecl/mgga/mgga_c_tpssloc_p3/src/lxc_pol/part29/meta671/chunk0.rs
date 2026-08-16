//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2244/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2244<F: Float>(t1361: F, t16153: F, t26288: F, t1339: F, t16206: F, t6936: F, t1825: F, t22827: F, t3719: F, t1307: F, t7708: F, t80840: F, t90787: F) -> (F, F, F, F) {
    let t91333 = t26288 * t1361 * t16153;
    let t91336 = t6936 * t1339 * t16206;
    let t91340 = t22827 * t1339 * t1825 * t3719;
    let t91344 = t80840 * t90787 * t7708 * t1307;
    (t91333, t91336, t91340, t91344)
}
