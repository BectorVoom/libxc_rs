//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2530/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2530<F: Float>(t10186: F, t13785: F, t13839: F, t2986: F, t42837: F, t10236: F, t12652: F, t12648: F, t13783: F, t1597: F, t10237: F, t340: F, t4548: F, t698: F, t973: F) -> (F, F, F, F, F, F, F) {
    let t48244 = t10186 * t13785;
    let t48250 = t2986 * t42837 * t13839;
    let t48256 = t10236 * t12652;
    let t48269 = t10236 * t12648;
    let t48279 = t13783 * t1597;
    let t48281 = t2986 * t48279 * t10237;
    let t48292 = t973 * t698 * t340 * t4548;
    (t48244, t48250, t48256, t48269, t48279, t48281, t48292)
}
