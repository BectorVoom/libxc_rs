//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1118/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1118<F: Float>(t12817: F, t22917: F, t5211: F, t41095: F, t950: F, t5218: F, t7149: F, t7049: F, t12599: F, t24835: F, t30170: F, t3406: F) -> (F, F, F, F, F) {
    let t47832 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t5211 * t22917 * t12817;
    let t47833 = t41095 * t950;
    let t47836 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t5218 * t7149 * t47833;
    let t47839 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t5218 * t7049 * t47833;
    let t47841 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t24835 * t12599;
    let t47844 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t5211 * t30170 * t3406;
    (t47832, t47836, t47839, t47841, t47844)
}
