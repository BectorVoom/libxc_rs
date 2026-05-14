//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 270/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk270<F: Float>(t3052: F, t378: F, t3051: F, t1639: F, t1640: F, t3042: F, t3045: F, t3048: F, t35: F, t374: F, t1594: F, t3037: F, t25: F, t938: F, t373: F, t383: F) -> (F, F, F, F, F, F, F) {
    let t3053 = t378 * t3052;
    let t3054 = t3051 * t3053;
    let t3056 = t1639 + t1640 / 9.0 + t3042 / 9.0 - 2.0 / 9.0 * t3045 + 2.0 / 3.0 * t3048 - 2.0 / 3.0 * t3054;
    let t3057 = t3056 * t35;
    let t3058 = t374 * t3057;
    let t3061 = t1594 * t3037;
    let t3064 = t938 * t25;
    let t3065 = t373 * t383;
    (t3054, t3056, t3057, t3058, t3061, t3064, t3065)
}
