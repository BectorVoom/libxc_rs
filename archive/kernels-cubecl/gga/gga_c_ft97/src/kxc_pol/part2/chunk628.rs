//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 628/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk628<F: Float>(t30: F, t7911: F, t25: F, t1663: F, t37: F, t78: F, t23: F, t2999: F, t26: F, t1771: F, t380: F, t1644: F, t458: F) -> (F, F, F, F, F, F, F) {
    let t7913 = F::cast_from(1.0_f64) / t30 / t7911;
    let t7914 = t25 * t7913;
    let t7918 = t37 * t1663;
    let t7919 = t7918 * t78;
    let t7943 = t2999 * t23;
    let t7944 = t26 * t7943;
    let t7945 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t7944;
    let t7946 = t1771 * t380;
    let t7948 = t458 * t1644;
    (t7914, t7919, t7943, t7944, t7945, t7946, t7948)
}
