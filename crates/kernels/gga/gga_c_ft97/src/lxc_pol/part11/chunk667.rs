//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 667/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk667<F: Float>(t9707: F, t9708: F, t27: F, t89: F, t2371: F, t2459: F, t713: F, t193: F, t9567: F, t241: F, t9570: F, t9571: F, t2336: F, t2366: F, t2344: F, t375: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9709 = t9707 * t9708;
    let t9711 = t89 * t27 * t9709;
    let t9713 = t2371 * t713 * t2459;
    let t9715 = t89 * t193 * t9713;
    let t9716 = t27 * t9567;
    let t9717 = t241 * t9570;
    let t9718 = t9717 * t9571;
    let t9720 = t89 * t9716 * t9718;
    let t9723 = t89 * t2336 * t2366;
    let t9725 = t375 * t2344;
    (t9709, t9711, t9713, t9715, t9716, t9717, t9718, t9720, t9723, t9725)
}
