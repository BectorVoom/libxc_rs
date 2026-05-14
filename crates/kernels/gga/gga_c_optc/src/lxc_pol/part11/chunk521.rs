//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 521/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk521<F: Float>(t2849: F, t496: F, t1514: F, t6: F, t1583: F, t1582: F) -> (F, F, F) {
    let t4290 = t496 * t2849;
    let t4295 = t1514 * t6;
    let t4296 = t1583 * t4295;
    let t4297 = t1582 * t4296;
    (t4290, t4296, t4297)
}
