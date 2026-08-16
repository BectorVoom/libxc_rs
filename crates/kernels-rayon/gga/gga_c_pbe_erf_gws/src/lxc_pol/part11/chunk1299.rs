//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1299/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1299(t10021: f64, t10025: f64, t13726: f64, t16356: f64, t16358: f64, t16362: f64, t16363: f64, t16366: f64, t16368: f64, t16369: f64, t16370: f64, t16371: f64, t16372: f64, t7997: f64, t8004: f64, t8012: f64, t8014: f64) -> f64 {
    let t50805 = 12.0_f64 * t7997 - 36.0_f64 * t13726 - 0.70178680769462448852e1_f64 * t10021 - 0.49291594608080000001e1_f64 * t10025 - t16356 - t16358 + 0.29298488058805055905e-2_f64 * t8004 - t16362 - t16363 + t16366 - t16368 + t16369 + t16370 - t16371 - t16372 - 96.0_f64 * t8012 + 144.0_f64 * t8014;
    t50805
}
