//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 732/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk732<F: Float>(t2493: F, t9761: F, t2512: F, t458: F, t249: F, t3051: F, t1771: F, t745: F, t738: F, t8608: F, t737: F, t2: F, t7514: F) -> (F, F, F, F, F, F, F) {
    let t9931 = t2493 * t9761;
    let t9933 = t458 * t2512;
    let t9935 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t3051 * t249;
    let t9936 = t1771 * t745;
    let t9938 = t738 * t8608;
    let t9939 = t737 * t9938;
    let t9942 = t7514 * t2;
    (t9931, t9933, t9935, t9936, t9938, t9939, t9942)
}
