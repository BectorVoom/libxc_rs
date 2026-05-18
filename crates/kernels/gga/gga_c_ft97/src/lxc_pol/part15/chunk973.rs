//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 973/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk973<F: Float>(t21400: F, t375: F, t89: F, t21431: F, t41962: F, t1882: F, t21519: F, t21736: F, t21728: F, t21740: F, t21533: F, t21661: F, t8392: F) -> (F, F, F, F, F, F, F, F) {
    let t81124 = t89 * t375 * t21400;
    let t81131 = t89 * t41962 * t21431;
    let t81162 = t1882 * t21519;
    let t81164 = t1882 * t21736;
    let t81170 = t1882 * t21728;
    let t81183 = t1882 * t21740;
    let t81207 = t1882 * t21533;
    let t81209 = t8392 * t21661;
    (t81124, t81131, t81162, t81164, t81170, t81183, t81207, t81209)
}
