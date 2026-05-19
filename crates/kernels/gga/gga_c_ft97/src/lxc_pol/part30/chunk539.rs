//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 539/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk539<F: Float>(t2387: F, t24275: F, t679: F, t703: F, t420: F, t230: F, t626: F, t1418: F, t1417: F, t1609: F, t218: F, t6: F, t9681: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t24276 = t2387 * t24275;
    let t24277 = t703 * t679;
    let t24278 = t420 * t24277;
    let t24286 = t626 * t230;
    let t24287 = t1418 * t24286;
    let t24289 = F::cast_from(0.42562405586419753087e-2_f64) * t1417 * t24287;
    let t24310 = t1609 * sigma2;
    let t24311 = t24310 * t218;
    let t24322 = t9681 * t6;
    (t24276, t24278, t24286, t24287, t24289, t24311, t24322)
}
