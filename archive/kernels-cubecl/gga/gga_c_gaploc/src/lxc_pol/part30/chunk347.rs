//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 347/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk347<F: Float>(t1508: F, t189: F, t200: F, t23: F, t203: F, t61: F, t172: F, t911: F, t107: F, t1328: F, t600: F, t568: F) -> (F, F, F, F, F, F) {
    let t1509 = t1508 * t189;
    let t1512 = t23 * t200;
    let t1514 = t61 * t1512 * t203;
    let t1519 = t911 * t172;
    let t1520 = t107 * t1519;
    let t1525 = t600 * t1328;
    let t1526 = t568 * t1525;
    (t1509, t1512, t1514, t1519, t1520, t1526)
}
