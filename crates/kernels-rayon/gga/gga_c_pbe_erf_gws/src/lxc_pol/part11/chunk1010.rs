//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1010/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1010(t12729: f64, t597: f64, t12709: f64, t649: f64, t1017: f64, t3346: f64, t12817: f64, t17870: f64, t639: f64, t10743: f64, t10878: f64, t12730: f64, t561: f64, t582: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40676 = t597 * t12729;
    let t40687 = t649 * t12709;
    let t40696 = t3346 * t1017;
    let t40718 = t639 * t17870 * t12817;
    let t40761 = t10743 * t10878;
    let t40764 = t561 * t582 * t12730;
    (t40676, t40687, t40696, t40718, t40761, t40764)
}
