//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 883/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk883<F: Float>(t40367: F, t40403: F, t40446: F, t40486: F, t579: F, t91: F, t2120: F, t2086: F, t3000: F, t520: F, t89: F, t1975: F, t7773: F, t1636: F, t2076: F, t375: F, t9008: F) -> (F, F, F, F, F, F) {
    let t40490 = t91 * t579 * (t40367 + t40403 + t40446 + t40486);
    let t40492 = t2120 * t2120;
    let t40494 = t91 * t2086 * t40492;
    let t40497 = t89 * t3000 * t520;
    let t40500 = t89 * t7773 * t1975;
    let t40503 = t89 * t1636 * t2076;
    let t40506 = t89 * t375 * t9008;
    (t40490, t40494, t40497, t40500, t40503, t40506)
}
