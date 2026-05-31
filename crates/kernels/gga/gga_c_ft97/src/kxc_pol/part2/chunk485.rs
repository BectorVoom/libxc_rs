//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 485/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk485<F: Float>(t2832: F, t295: F, t312: F, t681: F, t865: F, t89: F, t311: F, t869: F) -> (F, F, F) {
    let t2834 = t295 * t2832 * t312;
    let t2839 = t89 * t681 * t865;
    let t2842 = F::cast_from(1.0_f64) / t869 / t311;
    (t2834, t2839, t2842)
}
