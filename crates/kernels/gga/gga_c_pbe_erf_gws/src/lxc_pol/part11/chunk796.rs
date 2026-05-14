//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 796/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk796<F: Float>(t13280: F, t13345: F, t13392: F, t13453: F, t13484: F, t13515: F, t13568: F, t13603: F, t898: F, t338: F, t353: F, t1161: F, t3721: F, t6126: F, t1144: F, t3722: F) -> (F, F, F, F, F, F, F) {
    let t13606 = t13280 + t13345 + t13392 + t13453 + t13484 + t13515 + t13568 + t13603;
    let t13607 = t898 * t13606;
    let t13609 = t338 * t353 * t13607;
    let t13612 = t3721 * t1161;
    let t13613 = t6126 * t13612;
    let t13615 = t338 * t353 * t13613;
    let t13619 = t338 * t1144 * t3722;
    (t13606, t13607, t13609, t13612, t13613, t13615, t13619)
}
