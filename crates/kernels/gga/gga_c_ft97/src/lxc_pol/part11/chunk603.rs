//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 603/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk603<F: Float>(t488: F, t8355: F, t83: F, t1841: F, t487: F, t492: F, t1820: F, t1825: F, t1851: F, t1853: F, t379: F, t1909: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8356 = t488 * t8355;
    let t8357 = t83 * t8356;
    let t8360 = t1841 * t487;
    let t8361 = t8360 * t492;
    let t8362 = t83 * t8361;
    let t8364 = t1825 * t1820;
    let t8365 = t83 * t8364;
    let t8367 = t1851 * t1853;
    let t8368 = t8367 * t379;
    let t8369 = t1909 * t8368;
    (t8356, t8357, t8360, t8361, t8362, t8364, t8365, t8367, t8368, t8369)
}
