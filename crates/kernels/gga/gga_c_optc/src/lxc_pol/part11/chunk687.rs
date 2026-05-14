//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 687/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk687<F: Float>(t1772: F, t312: F, t310: F, t307: F, t7253: F, t7256: F, t906: F, t317: F, t1: F, t2672: F) -> (F, F, F, F, F, F, F, F) {
    let t7894 = t1772 * t312;
    let t7895 = t310 * t7894;
    let t7897 = 0.80492236016562572729e-3 * t307 * t7895;
    let t7924 = t7253 * t7256;
    let t7946 = t906 * t906;
    let t7947 = 1.0 / t7946;
    let t7948 = t317 * t7947;
    let t8002 = t2672 * t1;
    (t7894, t7895, t7897, t7924, t7946, t7947, t7948, t8002)
}
