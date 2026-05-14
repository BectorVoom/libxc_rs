//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 975/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk975<F: Float>(t444: F, t983: F, t14: F, t4494: F, t26: F, t4635: F, t30: F, t4827: F, t1447: F, t41: F, t31: F, t1410: F, t1448: F, t500: F, t8: F, t1697: F, t51: F) -> (F, F, F, F, F, F, F, F) {
    let t12973 = t983 * t444;
    let t13925 = t14 * t4494;
    let t14431 = t26 * t4635;
    let t16036 = t30 * t4827;
    let t16046 = 1.0 / t1447 / t41;
    let t16047 = t31 * t16046;
    let t16074 = t1410 * t1448;
    let t16089 = 1.0 / t14431;
    let t16111 = 1.0 / t13925;
    let t16129 = t8 * t500;
    let t16190 = t51 * t1697;
    (t12973, t16036, t16047, t16074, t16089, t16111, t16129, t16190)
}
