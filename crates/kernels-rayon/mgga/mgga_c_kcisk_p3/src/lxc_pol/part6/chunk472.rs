//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 472/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk472(t4029: f64, t346: f64, t344: f64, t347: f64, t4007: f64, t313: f64, t353: f64, t964: f64, t1311: f64, t24: f64, t1232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4030 = 1.0_f64 / t4029;
    let t4031 = t346 * t4030;
    let t4037 = 1.0_f64 / t347 / t344;
    let t4041 = 4.0_f64 / 9.0_f64 * t4007;
    let t4049 = 0.39862222222222222223e0_f64 * t4007;
    let t4054 = 1.0_f64/f64::sqrt(t344);
    let t4060 = t353 * t964 * t313;
    let t4061 = 0.27385555555555555555e0_f64 * t4060;
    let t4065 = t24 * t1311;
    let t4079 = t1232 * t1232;
    let t4080 = 1.0_f64 / t4079;
    let t4081 = t346 * t4080;
    (t4030, t4031, t4037, t4041, t4049, t4054, t4060, t4061, t4065, t4079, t4080, t4081)
}
