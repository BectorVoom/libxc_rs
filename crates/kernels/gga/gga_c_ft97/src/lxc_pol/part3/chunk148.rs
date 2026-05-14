//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 148/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk148<F: Float>(t363: F, t423: F, t420: F, t419: F, t412: F, t417: F) -> (F, F, F) {
    let t424 = t423 * t363;
    let t425 = t420 * t424;
    let t426 = t419 * t425;
    let t428 = -0.51074886703703703704e-1 * t412 + t417 + 0.6384360837962962963e-2 * t426;
    (t424, t426, t428)
}
