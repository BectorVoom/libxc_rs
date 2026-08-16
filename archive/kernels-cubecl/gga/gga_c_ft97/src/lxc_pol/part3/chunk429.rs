//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 429/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk429<F: Float>(t3056: F, t35: F, t374: F, t1594: F, t3037: F, t25: F, t938: F, t373: F, t383: F, t408: F, t401: F, t64: F, t66: F) -> (F, F, F, F, F, F, F) {
    let t3057 = t3056 * t35;
    let t3058 = t374 * t3057;
    let t3061 = t1594 * t3037;
    let t3064 = t938 * t25;
    let t3065 = t373 * t383;
    let t3066 = t3065 * t35;
    let t3067 = t3064 * t3066;
    let t3070 = t408 * t938;
    let t3071 = t3070 * t401;
    let t3076 = t64 * t66;
    (t3057, t3058, t3061, t3066, t3067, t3071, t3076)
}
