//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1388/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1388<F: Float>(t126463: F, t2665: F, t446: F, t10409: F, t126467: F, t1234: F, t193: F, t28719: F, t6308: F, t852: F, t1476: F, t19240: F, t1486: F, t2781: F, t10248: F, t126795: F) -> (F, F, F, F, F, F) {
    let t127879 = t446 * t2665 * t126463;
    let t127882 = t446 * t10409 * t126467;
    let t127887 = t6308 * t193 * t852 * t28719 * t1234;
    let t127889 = t1476 * t19240;
    let t127892 = t1486 * t193 * t2781 * t127889;
    let t127894 = t446 * t10248 * t126795;
    (t127879, t127882, t127887, t127889, t127892, t127894)
}
