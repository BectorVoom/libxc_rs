//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 323/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk323<F: Float>(t3057: F, t374: F, t1594: F, t3037: F, t25: F, t938: F, t373: F, t383: F, t35: F) -> (F, F, F, F) {
    let t3058 = t374 * t3057;
    let t3061 = t1594 * t3037;
    let t3064 = t938 * t25;
    let t3065 = t373 * t383;
    let t3066 = t3065 * t35;
    (t3058, t3061, t3064, t3066)
}
