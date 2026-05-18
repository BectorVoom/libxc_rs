//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 633/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk633<F: Float>(t1725: F, t1732: F, t10: F, t3050: F, t83: F, t1636: F, t433: F, t89: F, t1756: F, t375: F, t1586: F, t355: F) -> (F, F, F, F, F, F) {
    let t8133 = t1725 * t1732;
    let t8189 = t10 * t3050 * t83;
    let t8190 = F::new(14.0) / F::new(81.0) * t8189;
    let t8192 = t89 * t1636 * t433;
    let t8195 = t89 * t375 * t1756;
    let t8216 = t355 * t1586;
    (t8133, t8189, t8190, t8192, t8195, t8216)
}
