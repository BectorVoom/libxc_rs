//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1009/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1009<F: Float>(t12350: F, t1620: F, t23207: F, t2677: F, t12752: F, t2612: F, t1815: F, t3469: F, t3553: F, t639: F, t1044: F, t12497: F, t5522: F, t11032: F, t3407: F, t12631: F, t7130: F) -> (F, F, F, F, F, F) {
    let t48213 = 64.0 / 9.0 * t1620 * t2677 * t23207 * t12350;
    let t48215 = 32.0 / 15.0 * t2612 * t12752;
    let t48219 = 16.0 / 15.0 * t639 * t1815 * t3469 * t3553;
    let t48223 = 32.0 / 9.0 * t639 * t5522 * t12497 * t1044;
    let t48225 = 16.0 / 15.0 * t11032 * t3407;
    let t48227 = 32.0 / 15.0 * t7130 * t12631;
    (t48213, t48215, t48219, t48223, t48225, t48227)
}
