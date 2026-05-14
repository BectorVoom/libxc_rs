//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 419/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk419<F: Float>(t2347: F, t312: F, t1250: F, t1882: F, t1234: F, t2755: F, t1228: F, t1775: F, t2: F, t2766: F, t848: F, t1232: F, t458: F, t1212: F, t4032: F, t4049: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4140 = t312 * t2347;
    let t4156 = t1882 * t1250;
    let t4191 = t2755 * t1234;
    let t4197 = t1775 * t1228;
    let t4199 = t2766 * t2;
    let t4206 = t848 * t2;
    let t4213 = t458 * t1232;
    let t4218 = t2 * t1212;
    let t4230 = t4032 / 27.0;
    let t4235 = t4049 / 9.0;
    (t4140, t4156, t4191, t4197, t4199, t4206, t4213, t4218, t4230, t4235)
}
