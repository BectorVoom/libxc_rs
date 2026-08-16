//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 269/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk269<F: Float>(t140: F, t1014: F, t133: F, t1010: F) -> (F, F) {
    let t141 = F::cast_from(0.1e-59_f64) < t140;
    let t1015 = t133 * t1014;
    let t1017 = piecewise3::<F>(t141, F::cast_from(2.0_f64) * t1010 - t1015, F::cast_from(0.0_f64));
    (t1015, t1017)
}
