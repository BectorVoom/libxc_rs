//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1050/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1050<F: Float>(t191: F, t33300: F, t2373: F, t27: F, t89: F, t1636: F, t2374: F, t2413: F, t2459: F, t2354: F, t446: F, t2405: F) -> (F, F, F, F, F, F, F) {
    let t41848 = t191 * t33300;
    let t41849 = t2373 * t2373;
    let t41852 = t89 * t27 * t41848 * t41849;
    let t41855 = t89 * t1636 * t2374;
    let t41856 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41855;
    let t41857 = t2413 * t2459;
    let t41859 = t446 * t2354 * t41857;
    let t41861 = t2405 * t2459;
    (t41849, t41852, t41855, t41856, t41857, t41859, t41861)
}
