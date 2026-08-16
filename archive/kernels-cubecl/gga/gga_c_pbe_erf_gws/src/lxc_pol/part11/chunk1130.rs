//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1130/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1130<F: Float>(t186: F, t211: F, t47948: F, t47973: F, t48008: F, t48037: F, t650: F, t32279: F, t41297: F, t41300: F, t47979: F, t639: F, t7853: F) -> (F, F, F, F, F) {
    let t48043 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t211 * t186 * t650 * (t47948 + t47973 + t48008 + t48037);
    let t48044 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t32279;
    let t48045 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t41297;
    let t48046 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t41300;
    let t48049 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t639 * t7853 * t47979;
    (t48043, t48044, t48045, t48046, t48049)
}
