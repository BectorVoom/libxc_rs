//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 513/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk513<F: Float>(t327: F, t703: F, t230: F, t113: F, t332: F, t38: F, t401: F, t6: F, t77: F, t51: F, t78: F, t388: F, t408: F, t1693: F, t1710: F, t139: F, rho0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4334 = t703 * t327;
    let t4342 = t230 * t327;
    let t4381 = t332 * t113;
    let t5517 = t38 * t401;
    let t5536 = t77 * t6;
    let t5537 = t5536 * t51;
    let t5544 = t78 * t6;
    let t5545 = t388 * t5544;
    let t5566 = t408 * t6;
    let t5588 = t1693 * rho0;
    let t5596 = t1710 * t6;
    let t5784 = t139 * t6;
    (t4334, t4342, t4381, t5517, t5537, t5544, t5545, t5566, t5588, t5596, t5784)
}
