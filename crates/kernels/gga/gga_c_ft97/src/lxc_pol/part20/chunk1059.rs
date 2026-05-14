//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1059/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1059<F: Float>(t108080: F, t1434: F, t27842: F, t681: F, t108045: F, t446: F, t9770: F, t1131: F, t24395: F, t193: F, t2506: F, t3821: F, t6061: F, t13852: F, t6135: F, t41825: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t108081 = t108080 / 6.0;
    let t108083 = t1434 * t681 * t27842;
    let t108084 = 2.0 / 3.0 * t108083;
    let t108086 = t446 * t9770 * t108045;
    let t108088 = t24395 * t1131;
    let t108091 = t1434 * t193 * t2506 * t108088;
    let t108092 = t6061 * t3821;
    let t108095 = t1434 * t193 * t2506 * t108092;
    let t108097 = t6135 * t13852;
    let t108099 = t446 * t41825 * t108097;
    (t108081, t108083, t108084, t108086, t108088, t108091, t108092, t108095, t108097, t108099)
}
