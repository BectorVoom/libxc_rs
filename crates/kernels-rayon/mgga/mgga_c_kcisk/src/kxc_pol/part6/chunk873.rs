//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 873/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk873(t1814: f64, t28571: f64, t28368: f64, t682: f64, t11460: f64, t11495: f64, t1809: f64, t23068: f64, t23070: f64, t23072: f64, t23074: f64, t23118: f64, t2505: f64, t28532: f64, t28539: f64, t674: f64, t702: f64, t8616: f64) -> f64 {
    let t28572 = t1814 * t28571;
    let t28575 = t682 * t28368;
    let t28578 = -3.0_f64 * t8616 * t2505 - t28532 * t702 - 0.14055920378328537299e-1_f64 * t23070 + 0.70279601891642686494e-2_f64 * t23072 - 0.42167761134985611897e-1_f64 * t23074 + 0.14055920378328537299e-1_f64 * t23068 - 0.28111840756657074597e-1_f64 * t23118 - t11460 - 0.23426533963880895498e-2_f64 * t1809 * t28539 - 0.46853067927761790996e-2_f64 * t674 * t28572 - 0.14055920378328537299e-1_f64 * t11495 * t28575;
    t28578
}
