//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 736/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk736<F: Float>(t3172: F, t376: F, t89: F, t1755: F, t979: F, t452: F, t488: F, t3052: F, t447: F, t499: F, t1637: F, t973: F) -> (F, F, F, F) {
    let t11567 = F::new(2.0) / F::new(9.0) * t89 * t376 * t3172;
    let t11568 = t979 * t1755;
    let t11570 = t452 * t488 * t11568;
    let t11574 = t447 * t499 * t3052;
    let t11578 = t89 * t1637 * t973;
    (t11567, t11570, t11574, t11578)
}
