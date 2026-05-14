//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 965/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk965<F: Float>(t30407: F, t3465: F, t3553: F, t5522: F, t639: F, t1044: F, t12493: F, t17331: F, t184: F, t199: F, t3397: F, t3486: F, t16782: F, t40422: F, t587: F, t950: F) -> (F, F, F, F, F) {
    let t47507 = 32.0 / 45.0 * t30407;
    let t47511 = 8.0 / 9.0 * t639 * t5522 * t3465 * t3553;
    let t47515 = 128.0 / 81.0 * t639 * t17331 * t12493 * t1044;
    let t47519 = 8.0 / 5.0 * t3397 * t3486 * t184 * t199;
    let t47523 = 32.0 / 15.0 * t587 * t16782 * t40422 * t950;
    (t47507, t47511, t47515, t47519, t47523)
}
