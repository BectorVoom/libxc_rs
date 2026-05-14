//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 650/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk650<F: Float>(t6329: F, t2229: F, t740: F, t1820: F, t1828: F, t1859: F, t1867: F, t586: F, t1757: F, t1784: F, t535: F, t31: F, t3648: F, t4: F, t14: F, t2: F, t25: F) -> (F, F, F, F, F, F, F, F) {
    let t6330 = 12.0 * t6329;
    let t6332 = 7.0 / 2.0 * t2229 * t740;
    let t6343 = t1820 * t1828;
    let t6347 = t1859 * t1867;
    let t6348 = t6347 * t586;
    let t6356 = 6.0 * t1757 * t535 * t1784;
    let t6359 = 0.34451131037037037036e-2 * t4 * t3648 * t31;
    let t6363 = 1.0 / t14 / t25 * t2 / 4.0;
    (t6330, t6332, t6343, t6347, t6348, t6356, t6359, t6363)
}
