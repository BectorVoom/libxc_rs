//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 948/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk948<F: Float>(t1775: F, t9900: F, t2494: F, t8282: F, t11176: F, t249: F, t3051: F, t745: F, t1771: F, t2508: F, t2512: F, t458: F, t9944: F, t2: F, t33300: F, t9965: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42192 = t1775 * t9900;
    let t42194 = t8282 * t2494;
    let t42206 = 280.0 / 81.0 * t11176 * t249;
    let t42207 = t3051 * t745;
    let t42212 = t1771 * t2508;
    let t42214 = t1771 * t2512;
    let t42216 = t458 * t9944;
    let t42218 = t33300 * t2;
    let t42227 = t458 * t9965;
    (t42192, t42194, t42206, t42207, t42212, t42214, t42216, t42218, t42227)
}
