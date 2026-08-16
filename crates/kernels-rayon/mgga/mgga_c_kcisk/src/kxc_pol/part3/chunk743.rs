//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 743/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk743(t11394: f64, t11480: f64, t1824: f64, t4684: f64, t5101: f64, t1850: f64, t5090: f64, t11460: f64, t11461: f64, t11463: f64, t11465: f64, t11467: f64, t11469: f64, t11472: f64, t11476: f64, t1809: f64, t674: f64) -> f64 {
    let t11481 = t11480 * t11394;
    let t11485 = t5101 * t1824 * t4684;
    let t11488 = t1850 * t5090;
    let t11490 = -t11460 - 0.14055920378328537299e-1_f64 * t11461 - 0.28111840756657074597e-1_f64 * t11463 + 0.70279601891642686494e-2_f64 * t11465 + 0.14055920378328537299e-1_f64 * t11467 - 0.23426533963880895498e-2_f64 * t1809 * t11469 - 0.46853067927761790996e-2_f64 * t674 * t11472 - 0.42167761134985611897e-1_f64 * t1809 * t11476 - 0.56223681513314149196e-1_f64 * t674 * t11481 + 0.42167761134985611897e-1_f64 * t674 * t11485 - 0.14055920378328537299e-1_f64 * t11488;
    t11490
}
