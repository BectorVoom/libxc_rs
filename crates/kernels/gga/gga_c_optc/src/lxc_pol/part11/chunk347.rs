//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 347/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk347<F: Float>(t1192: F, t1194: F, t1444: F, t1457: F, t1201: F, t1203: F) -> (F, F) {
    let t1561 = -t1192 - 0.19388333333333333333e1 * t1444 - t1194 - 0.12315e-2 * t1457;
    let t1565 = -t1201 - 0.72691666666666666667e3 * t1444 - t1203 - 0.78666666666666666667e2 * t1457;
    (t1561, t1565)
}
