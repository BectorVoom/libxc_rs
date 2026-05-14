//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 796/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk796<F: Float>(t9721: F, t16344: F, t16345: F, t16346: F, t16347: F, t6480: F, t6484: F, t6488: F, t6492: F, t6816: F, t6823: F, t6827: F, t6840: F, t16333: F, t16334: F, t16343: F) -> (F, F) {
    let t16348 = 24.0 * t9721;
    let t16349 = t16344 + t6816 - t16345 - t6480 - t6484 + t6488 - t6823 + t6827 + t16346 - t16347 - t16348 + t6492 - t6840;
    let t16351 = t16333 + t16334 + t16343 + t16349;
    (t16348, t16351)
}
