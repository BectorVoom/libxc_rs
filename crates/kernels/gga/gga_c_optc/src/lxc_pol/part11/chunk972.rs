//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 972/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk972<F: Float>(t1319: F, t188: F, t21979: F, t2229: F, t3563: F, t1245: F, t6316: F, t6319: F, t1281: F, t6976: F, t1287: F, t23065: F, t1291: F, t23163: F, t1278: F, t23171: F) -> (F, F, F, F, F, F, F, F) {
    let t28626 = t188 * t21979 * t1319;
    let t28628 = t2229 * t3563;
    let t28635 = t6316 * t1245;
    let t28637 = t6319 * t1245;
    let t28700 = t1281 * t6976;
    let t29117 = t23065 * t1287;
    let t29128 = t23163 * t1291;
    let t29210 = t23171 * t1278;
    (t28626, t28628, t28635, t28637, t28700, t29117, t29128, t29210)
}
