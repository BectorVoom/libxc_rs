//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 233/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk233<F: Float>(t250: F, t251: F, t1771: F, t249: F, t1775: F, t740: F, t458: F, t745: F, t2344: F, t241: F) -> (F, F, F, F, F) {
    let t2475 = F::cast_from(1.0_f64) / t251 / t250;
    let t2481 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1771 * t249;
    let t2482 = t1775 * t740;
    let t2484 = t458 * t745;
    let t2486 = t2344 * t241;
    (t2475, t2481, t2482, t2484, t2486)
}
