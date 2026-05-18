//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 614/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk614<F: Float>(t83: F, t8502: F, t463: F, t480: F, t1912: F, t1820: F, t487: F, t379: F, t1909: F, t1637: F, t482: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t8503 = t83 * t8502;
    let t8506 = t463 * t480;
    let t8507 = t8506 * t1912;
    let t8510 = t487 * t1820;
    let t8511 = t8510 * t379;
    let t8512 = t1909 * t8511;
    let t8516 = t89 * t1637 * t482;
    (t8503, t8506, t8507, t8510, t8511, t8512, t8516)
}
