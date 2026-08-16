//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1300/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1300(t24143: f64, t3: f64, t4002: f64, t6012: f64, t10288: f64, t10293: f64, t2002: f64, t2028: f64, t20292: f64, t24140: f64, t24142: f64, t24149: f64, t24154: f64, t24158: f64, t24161: f64, t24163: f64, t24186: f64, t24205: f64, t27275: f64, t3171: f64, t3925: f64, t572: f64, t8296: f64) -> f64 {
    let t28258 = t24143 * t3;
    let t28268 = t6012 * t4002;
    let t28274 = -2.0_f64 / 81.0_f64 * t24140 - 5.0_f64 / 243.0_f64 * t572 * t8296 * t10288 * t2002 - 40.0_f64 / 729.0_f64 * t572 * t24205 * t20292 * t3925 * t2028 + 2.0_f64 / 27.0_f64 * t572 * t3171 * t10293 * t2002 + 40.0_f64 / 243.0_f64 * t27275 * t24154 * t28258 - 16.0_f64 / 27.0_f64 * t27275 * t24149 * t28258 + 8.0_f64 / 9.0_f64 * t27275 * t24142 * t28258 - 4.0_f64 / 729.0_f64 * t28268 - 8.0_f64 / 243.0_f64 * t24158 - 2.0_f64 / 81.0_f64 * t24161 + 8.0_f64 / 81.0_f64 * t24163 + 28.0_f64 / 729.0_f64 * t24186;
    t28274
}
