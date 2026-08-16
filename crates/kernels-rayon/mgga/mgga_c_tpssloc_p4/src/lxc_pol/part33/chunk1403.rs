//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1403/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1403(t20356: f64, t6637: f64, t6968: f64, t80732: f64, t12250: f64, t1992: f64, t74967: f64, t81027: f64, t22897: f64, t3792: f64, t107397: f64, t107402: f64, t107406: f64, t107413: f64, t107417: f64, t1336: f64, t1814: f64, t2013: f64, t20595: f64, t26403: f64, t28152: f64, t28156: f64, t28178: f64, t5234: f64, t5344: f64, t6388: f64, t6415: f64, t91029: f64, t91078: f64, t91081: f64, t97179: f64, t97200: f64, t97494: f64) -> f64 {
    let t107431 = t80732 * t6637 * t6968 * t20356;
    let t107435 = t1992 * t81027 * t74967 * t12250;
    let t107439 = t1992 * t22897 * t74967 * t3792;
    let t107442 = 6.0_f64 * t1336 * t91029 * t6388 - 0.24674011002723396548e-1_f64 * t107397 - 0.34543615403812755166e0_f64 * t97179 + 0.82246703342411321825e-2_f64 * t107402 - 0.82246703342411321825e-2_f64 * t107406 + 3.0_f64 * t1814 * t28156 - 0.57572692339687925277e-1_f64 * t97200 - 0.24674011002723396548e-1_f64 * t107413 + 0.14804406601634037928e0_f64 * t107417 - 3.0_f64 * t5234 * t28152 - 0.78134368175290755733e-1_f64 * t91078 - 3.0_f64 * t5344 * t26403 * t6415 - 6.0_f64 * t5234 * t28178 + 0.49348022005446793095e-1_f64 * t91081 + 0.24674011002723396548e-1_f64 * t97494 - 0.19739208802178717238e0_f64 * t107431 - 0.49348022005446793095e-1_f64 * t107435 + 0.49348022005446793095e-1_f64 * t107439 + t20595 * t2013;
    t107442
}
