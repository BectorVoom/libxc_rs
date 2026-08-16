//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1004/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1004(t5445: f64, t723: f64, t17493: f64, t17498: f64, t17501: f64, t17503: f64, t17507: f64, t17511: f64, t17514: f64, t17517: f64, t17520: f64, t17523: f64, t5390: f64, t5451: f64) -> f64 {
    let t18296 = t5445 * t723;
    let t18300 = 8.0_f64 / 9.0_f64 * t18296 + 0.2e-20_f64 * t5451 * t5390 - t17493 - t17498 - t17501 + t17503 + t17507 + t17511 + t17514 + t17517 - t17520 + t17523;
    t18300
}
