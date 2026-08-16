//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1062/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1062(t7: f64, t10097: f64, t10136: f64, t10151: f64, t9861: f64, t214: f64, t4086: f64, t675: f64, t1289: f64, t1318: f64, t191: f64, t3984: f64, t2024: f64, t3926: f64, t6479: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t10154 = piecewise3(t9, 0.0_f64, t9861 + t10097 + t10136 + t10151);
    let t10158 = t4086 * t214;
    let t10159 = t10158 * t675;
    let t10163 = t1289 * t1318;
    let t10164 = t10163 * t675;
    let t10168 = t191 * t3984;
    let t10169 = t10168 * t675;
    let t10174 = t2024 * t6479 * t3926;
    (t10154, t10158, t10159, t10163, t10164, t10168, t10169, t10174)
}
