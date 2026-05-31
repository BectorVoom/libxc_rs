//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 109/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk109<F: Float>(t10: F, t16: F, t296: F) -> (F, F, F) {
    let t298 = t10 * t16 * t296;
    let t299 = t298 / F::cast_from(6.0_f64);
    let t300 = F::cast_from(10000000.0_f64) <= t299;
    let t301 = xc_e1_scaled::<F>(t299);
    (t298, t301, t299)
}
