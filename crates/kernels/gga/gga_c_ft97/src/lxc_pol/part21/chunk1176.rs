//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1176/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1176<F: Float>(t116749: F, t28: F, t432: F, t89: F, t16120: F, t5674: F, t5675: F, t8411: F, t1871: F, t22952: F, t23057: F, t29669: F, t3157: F, t942: F, t100453: F, t101733: F, t101734: F, t25985: F) -> (F, F, F, F, F) {
    let t116752 = t89 * t28 * t116749 * t432;
    let t116756 = t5674 * t8411 * t5675 * t16120;
    let t116760 = t22952 * t1871 * t23057 * t29669;
    let t116764 = t22952 * t1871 * t5675 * t3157 * t942;
    let t116767 = t101733 * t100453 * t101734 * t25985;
    (t116752, t116756, t116760, t116764, t116767)
}
