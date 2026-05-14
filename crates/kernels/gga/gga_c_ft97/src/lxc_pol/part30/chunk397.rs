//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 397/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk397<F: Float>(t6118: F, t6884: F, t2506: F, t6852: F, t1434: F, t193: F, t6837: F, t743: F, t6135: F, t992: F, t2354: F, t446: F, t1131: F, t6008: F, t89: F, t676: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6885 = t6118 * t6884;
    let t6887 = t2506 * t6852;
    let t6889 = t1434 * t193 * t6887;
    let t6891 = t743 * t6837;
    let t6893 = t1434 * t193 * t6891;
    let t6895 = t6135 * t992;
    let t6896 = t2354 * t6895;
    let t6897 = t446 * t6896;
    let t6899 = t6008 * t1131;
    let t6900 = t193 * t6899;
    let t6901 = t89 * t6900;
    let t6903 = t676 * t6837;
    (t6885, t6887, t6889, t6891, t6893, t6895, t6896, t6897, t6899, t6900, t6901, t6903)
}
