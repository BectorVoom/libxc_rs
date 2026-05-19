//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 645/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk645<F: Float>(t2268: F, t8675: F, t2253: F, t2273: F, t2281: F, t71: F, t118: F, t7911: F, t7944: F, t2296: F, t3626: F, t70: F) -> (F, F, F, F, F, F, F) {
    let t8676 = t8675 * t2268;
    let t8678 = t2253 * t2273;
    let t8680 = t71 * t2281;
    let t8690 = F::new(1.0) / t118 / t7911;
    let t8698 = F::cast_from(0.44934037037037037036e0_f64) * t7944;
    let t8714 = t2253 * t2296;
    let t8715 = t3626 * t70;
    (t8676, t8678, t8680, t8690, t8698, t8714, t8715)
}
