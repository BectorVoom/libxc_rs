//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 885/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk885<F: Float>(t4387: F, t8498: F, t3133: F, t8488: F, t3132: F, t1137: F, t7878: F, t1133: F, t2586: F, t3156: F, t1135: F, t2849: F) -> (F, F, F, F, F, F, F) {
    let t8499 = t4387 * t8498;
    let t8502 = t8488 * t3133;
    let t8503 = t3132 * t8502;
    let t8505 = t7878 * t1137;
    let t8506 = t1133 * t8505;
    let t8508 = t2586 * t3156;
    let t8509 = t1133 * t8508;
    let t8511 = t1135 * t2849;
    (t8499, t8503, t8505, t8506, t8508, t8509, t8511)
}
