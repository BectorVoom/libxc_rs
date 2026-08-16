//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 909/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk909(t16762: f64, t7115: f64, t7759: f64, t1416: f64, t422: f64, t5211: f64, t617: f64, t7491: f64, t1407: f64, t418: f64, t5218: f64, t562: f64, t7049: f64) -> (f64, f64, f64) {
    let t17128 = 16.0_f64 / 9.0_f64 * t7115 * t7759 * t16762;
    let t17133 = 32.0_f64 / 9.0_f64 * t5211 * t7491 * t1416 * t617 * t422;
    let t17138 = 32.0_f64 / 9.0_f64 * t5218 * t7049 * t1407 * t562 * t418;
    (t17128, t17133, t17138)
}
