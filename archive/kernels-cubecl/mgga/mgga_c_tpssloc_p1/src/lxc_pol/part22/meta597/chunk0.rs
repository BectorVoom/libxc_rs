//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2118/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2118<F: Float>(t13797: F, t1597: F, t13783: F, t340: F, t4548: F, t698: F, t973: F, t10224: F, t4522: F, t13895: F, t2960: F, t1599: F, t2402: F) -> (F, F, F, F, F, F) {
    let t48221 = t13797 * t1597;
    let t48279 = t13783 * t1597;
    let t48292 = t973 * t698 * t340 * t4548;
    let t48293 = F::cast_from(0.55555555555555555554e-3_f64) * t48292;
    let t48320 = t973 * t10224 * t4522;
    let t48321 = F::cast_from(0.18518518518518518518e-3_f64) * t48320;
    let t48328 = t2960 * t13895;
    let t48329 = F::cast_from(0.49382716049382716048e-3_f64) * t48328;
    let t48336 = t973 * t2402 * t1599;
    (t48221, t48279, t48293, t48321, t48329, t48336)
}
