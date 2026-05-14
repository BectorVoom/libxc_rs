//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1166/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1166<F: Float>(t191: F, t4529: F, t34378: F, t34506: F, t10517: F, t7014: F, t10615: F, t31167: F, t6703: F, t8248: F, t30762: F, t30765: F, t14626: F, t1562: F, t3410: F, t10348: F, t8158: F) -> (F, F, F, F, F, F, F, F) {
    let t34507 = t191 * t4529;
    let t34510 = 0.85801175884441024004e1 * t34506 * t34507 * t34378;
    let t34512 = 0.87421871174939309262e2 * t7014 * t10517;
    let t34530 = t10615 * t31167;
    let t34531 = 0.44688112439813033337e-1 * t34530;
    let t34533 = 0.2780593662921699852e0 * t8248 * t6703;
    let t34535 = 0.25561950635947166452e0 * t30762;
    let t34536 = 0.25561950635947166452e0 * t30765;
    let t34541 = 0.30674340763136599741e1 * t1562 * t14626 * t3410;
    let t34548 = 0.14300195980740170668e1 * t8158 * t10348;
    (t34510, t34512, t34531, t34533, t34535, t34536, t34541, t34548)
}
