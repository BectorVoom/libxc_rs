//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 415/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk415<F: Float>(t193: F, t6109: F, t6879: F, t1091: F, t2354: F, t6119: F, t6118: F, t2506: F, t6852: F, t1434: F, t6837: F, t743: F) -> (F, F, F, F, F, F) {
    let t6881 = t6109 * t193 * t6879;
    let t6884 = t2354 * t6119 * t1091;
    let t6885 = t6118 * t6884;
    let t6887 = t2506 * t6852;
    let t6889 = t1434 * t193 * t6887;
    let t6891 = t743 * t6837;
    (t6881, t6884, t6885, t6887, t6889, t6891)
}
