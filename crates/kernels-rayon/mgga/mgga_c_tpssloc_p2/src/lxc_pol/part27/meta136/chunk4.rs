//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 778/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk778(t3016: f64, t974: f64, t2955: f64, t2958: f64, t2960: f64, t2969: f64, t2972: f64, t2975: f64, t2982: f64, t2986: f64, t2991: f64, t2996: f64, t3000: f64, t3011: f64, t346: f64, t973: f64, t980: f64, t987: f64) -> f64 {
    let t3017 = t974 * t3016;
    let t3020 = 0.81481481481481481481e-2_f64 * t2955 * t346 - 0.14814814814814814814e-2_f64 * t2958 - 0.14814814814814814814e-2_f64 * t2960 * t980 + 0.44444444444444444444e-2_f64 * t2960 * t987 - t2969 + 0.18518518518518518518e-3_f64 * t2972 - 0.55555555555555555554e-3_f64 * t2975 + 0.37037037037037037036e-3_f64 * t973 * t2982 - 0.55555555555555555554e-3_f64 * t2986 * t2991 - 0.55555555555555555554e-3_f64 * t973 * t2996 + 0.27777777777777777777e-3_f64 * t973 * t3000 - 0.83333333333333333332e-3_f64 * t973 * t3011 - 0.83333333333333333332e-3_f64 * t973 * t3017;
    t3020
}
