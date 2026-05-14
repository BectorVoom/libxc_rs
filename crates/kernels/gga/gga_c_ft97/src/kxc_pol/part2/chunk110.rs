//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 110/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk110<F: Float>(t2: F, t295: F, t192: F, t92: F, t91: F, t298: F) -> (F, F, F, F, F, F) {
    let t302 = t295 * t2;
    let t303 = t192 * t302;
    let t304 = t92 * t303;
    let t305 = f64::sqrt(t304);
    let t306 = t91 * t305;
    let t309 = 3.0 + t306 / 3.0 + t298 / 3.0;
    (t302, t303, t304, t305, t306, t309)
}
