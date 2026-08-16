//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1091/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1091<F: Float>(t1044: F, t12493: F, t17331: F, t639: F, t184: F, t199: F, t3397: F, t3486: F, t16782: F, t40422: F, t587: F, t950: F) -> (F, F, F) {
    let t47515 = F::cast_from(128.0_f64) / F::cast_from(81.0_f64) * t639 * t17331 * t12493 * t1044;
    let t47519 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3397 * t3486 * t184 * t199;
    let t47523 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t587 * t16782 * t40422 * t950;
    (t47515, t47519, t47523)
}
