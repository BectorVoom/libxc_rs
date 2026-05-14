//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 147/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk147<F: Float>(t170: F, t328: F, t626: F, t327: F, t668: F, t231: F, t505: F, t322: F, t70: F) -> (F, F, F, F) {
    let t892 = t170 * t626 * t328 / 6.0;
    let t893 = t327 * t668;
    let t895 = t231 * t893 * t505;
    let t898 = t70 * t322;
    (t892, t893, t895, t898)
}
