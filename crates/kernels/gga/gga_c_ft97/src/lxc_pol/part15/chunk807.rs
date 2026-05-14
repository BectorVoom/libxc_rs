//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 807/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk807<F: Float>(t4512: F, t8282: F, t4523: F, t1636: F, t4496: F, t89: F, t57435: F, t57491: F, t57527: F, t57620: F, t57718: F, t4545: F, t463: F, t1786: F, t4599: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t59104 = t8282 * t4512;
    let t59143 = t8282 * t4523;
    let t59170 = t89 * t1636 * t4496;
    let t59339 = 8.0 / 27.0 * t57435;
    let t59354 = 8.0 / 27.0 * t57491;
    let t59364 = 8.0 / 81.0 * t57527;
    let t59392 = 8.0 / 9.0 * t57620;
    let t59426 = 4.0 / 27.0 * t57718;
    let t59486 = 4.0 / 9.0 * t59170;
    let t59506 = t463 * t4545;
    let t59510 = t1786 * t4545;
    let t59623 = t8232 * t4599;
    (t59104, t59143, t59170, t59339, t59354, t59364, t59392, t59426, t59486, t59506, t59510, t59623)
}
