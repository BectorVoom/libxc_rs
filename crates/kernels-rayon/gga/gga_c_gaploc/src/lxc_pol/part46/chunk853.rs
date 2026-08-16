//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 853/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk853(t2487: f64, t41965: f64, t6711: f64, t204: f64, t2476: f64, t41839: f64, t40228: f64, t40234: f64, t40237: f64, t40239: f64, t40243: f64, t40249: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41968 = 0.43710935587469654631e2_f64 * t2487 * t6711 * t41965;
    let t41970 = t2476 * t204 * t41839;
    let t41972 = 0.29792074959875355558e-1_f64 * t40228;
    let t41973 = 0.89376224879626066674e-1_f64 * t40234;
    let t41974 = 0.59584149919750711116e-1_f64 * t40237;
    let t41975 = 0.29792074959875355558e-1_f64 * t40239;
    let t41976 = 0.29792074959875355558e-1_f64 * t40243;
    let t41978 = 0.17041300423964777634e0_f64 * t40249;
    (t41968, t41970, t41972, t41973, t41974, t41975, t41976, t41978)
}
