//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 924/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk924(t1728: f64, t3054: f64, t1068: f64, t1717: f64, t10108: f64, t10184: f64, t10187: f64, t1030: f64, t13665: f64, t13667: f64, t13668: f64, t13671: f64, t13674: f64, t13678: f64, t13682: f64, t13684: f64, t13686: f64, t13689: f64, t13787: f64, t13790: f64, t13791: f64, t14051: f64, t1745: f64, t3038: f64, t305: f64, t3061: f64, t313: f64, t3158: f64, t339: f64) -> f64 {
    let t14053 = t3054 * t1728;
    let t14055 = t1068 * t1717;
    let t14057 = -t13665 - t13667 + 0.46853067927761790996e-2_f64 * t3061 * t13668 + 0.18741227171104716398e-1_f64 * t10108 * t13671 + 0.46853067927761790996e-2_f64 * t1030 * t13674 + 0.18741227171104716398e-1_f64 * t3158 * t13678 - t13682 - t13684 - 0.93706135855523581992e-2_f64 * t10184 + 0.23426533963880895498e-1_f64 * t13686 + t13689 - t3038 * t1745 - 0.46853067927761790996e-2_f64 * t305 * t13787 - 0.18741227171104716398e-1_f64 * t13790 * t13791 - 0.46853067927761790996e-2_f64 * t3158 * t313 - t14051 * t339 - t10187 - 0.93706135855523581992e-2_f64 * t14053 - 0.46853067927761790996e-2_f64 * t14055;
    t14057
}
