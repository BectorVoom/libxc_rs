//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 567/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk567<F: Float>(t26: F, t7943: F, t1771: F, t380: F, t1644: F, t458: F, t1648: F, t1652: F, t17: F, t7760: F) -> (F, F, F, F, F, F, F) {
    let t7944 = t26 * t7943;
    let t7945 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t7944;
    let t7946 = t1771 * t380;
    let t7948 = t458 * t1644;
    let t7950 = t458 * t1648;
    let t7952 = t458 * t1652;
    let t7954 = t17 * t7760;
    (t7944, t7945, t7946, t7948, t7950, t7952, t7954)
}
