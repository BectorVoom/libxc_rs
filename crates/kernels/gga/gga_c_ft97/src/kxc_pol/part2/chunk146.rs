//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 146/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk146<F: Float>(t358: F, t422: F, t363: F, t420: F, t419: F, t412: F, t417: F) -> (F, F, F, F, F) {
    let t423 = t422 * t358;
    let t424 = t423 * t363;
    let t425 = t420 * t424;
    let t426 = t419 * t425;
    let t428 = -F::cast_from(0.51074886703703703704e-1_f64) * t412 + t417 + F::cast_from(0.6384360837962962963e-2_f64) * t426;
    (t423, t424, t425, t426, t428)
}
