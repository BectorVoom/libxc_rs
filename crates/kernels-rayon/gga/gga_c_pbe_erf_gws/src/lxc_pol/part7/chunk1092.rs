//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1092/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1092(t50: f64, t19072: f64, t19075: f64, t19077: f64, t19079: f64, t19081: f64, t19544: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t19551 = piecewise3(t51, 0.0_f64, -56.0_f64 / 81.0_f64 * t19072 + 16.0_f64 / 9.0_f64 * t19075 - 2.0_f64 / 3.0_f64 * t19077 - 8.0_f64 / 9.0_f64 * t19079 + 2.0_f64 / 3.0_f64 * t19081);
    let t19553 = t19544 / 2.0_f64 + t19551 / 2.0_f64;
    t19553
}
