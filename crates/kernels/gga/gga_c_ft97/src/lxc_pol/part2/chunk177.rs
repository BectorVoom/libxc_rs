//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 177/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk177<F: Float>(t140: F, t550: F, t554: F, t133: F, t399: F, t540: F, t543: F) -> (F,) {
    let t141 = 0.1e-59 < t140;
    let t555 = t550 * t554;
    let t558 = piecewise3(t141, 2.0 * t540 - 0.60409133884038297798e0 * t543 * t399 + 0.60409133884038297798e0 * t140 * t399 - t133 * t555, 0.0);
    (t558,)
}
