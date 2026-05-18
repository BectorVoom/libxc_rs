//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 141/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk141<F: Float>(t177: F, t501: F, t178: F, t400: F, t108: F, t75: F, t14: F, t1: F, t112: F, t3: F, t78: F, t110: F, t72: F) -> (F, F, F, F, F, F, F, F) {
    let t502 = t177 * t501;
    let t503 = t400 * t178;
    let t506 = t75 * t108;
    let t507 = t506 * t14;
    let t508 = t112 * t1;
    let t509 = t3 * t78;
    let t510 = t508 * t509;
    let t513 = t110 * t72;
    (t502, t503, t506, t507, t508, t509, t510, t513)
}
