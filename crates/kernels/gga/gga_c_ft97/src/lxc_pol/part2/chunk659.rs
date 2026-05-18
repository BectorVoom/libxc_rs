//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 659/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk659<F: Float>(t9437: F, t157: F, t1882: F, t2182: F, t2187: F, t2202: F, t161: F, t7943: F, t89: F, t2252: F, t342: F, t657: F) -> (F, F, F, F, F, F) {
    let t9438 = F::new(1.0) / t9437;
    let t9439 = t157 * t9438;
    let t9449 = t1882 * t2182;
    let t9451 = t1882 * t2187;
    let t9453 = t1882 * t2202;
    let t9457 = F::new(28.0) / F::new(81.0) * t89 * t7943 * t161;
    let t9482 = t342 * t2252 * t657 / F::new(18.0);
    (t9439, t9449, t9451, t9453, t9457, t9482)
}
