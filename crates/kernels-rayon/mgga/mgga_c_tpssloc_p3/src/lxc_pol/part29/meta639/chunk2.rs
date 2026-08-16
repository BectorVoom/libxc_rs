//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2102/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2102(t13377: f64, t1880: f64, t1894: f64, t214: f64, t22984: f64, t22992: f64, t22993: f64, t23009: f64, t25297: f64, t2617: f64, t4166: f64, t4234: f64, t812: f64, t81571: f64, t81592: f64, t87055: f64, t87059: f64, t87067: f64, t87068: f64, t87073: f64, t87076: f64, t87078: f64, t87080: f64, t87084: f64) -> f64 {
    let t87092 = t1880 * t214 * t1894 * t13377;
    let t87094 = 0.3289868133696452873e-1_f64 * t87055 - 0.9869604401089358619e-1_f64 * t87059 - 2.0_f64 * t2617 * t25297 - 2.0_f64 * t812 * t22992 * t4234 + t87067 - 0.26044789391763585244e-1_f64 * t87068 - 0.41123351671205660912e-2_f64 * t81571 - 2.0_f64 * t4166 * t22993 + t87073 - 0.49348022005446793095e-1_f64 * t87076 - 0.2302907693587517011e0_f64 * t87078 + 0.63969658155208805863e-1_f64 * t87080 + 0.3289868133696452873e-1_f64 * t87084 - t4166 * t22984 + 2.0_f64 * t4166 * t23009 - 0.76763589786250567036e-1_f64 * t81592 + 0.82246703342411321825e-2_f64 * t87092;
    t87094
}
