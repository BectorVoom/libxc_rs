//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 929/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk929<F: Float>(t1273: F, t2961: F, t4381: F, t2956: F, t4375: F, t909: F, t332: F, t505: F, t4380: F, t2957: F, t992: F, t4354: F, t8675: F) -> (F, F, F, F, F, F) {
    let t14390 = t1273 * t2961;
    let t14391 = t14390 * t4381;
    let t14394 = t1273 * t2956;
    let t14395 = t14394 * t4381;
    let t14402 = t4375 * t909;
    let t14403 = t14402 * t4381;
    let t14408 = t332 * t505;
    let t14409 = t4380 * t14408;
    let t14412 = t2957 * t992;
    let t14421 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t8675 * t4354;
    (t14391, t14395, t14403, t14409, t14412, t14421)
}
