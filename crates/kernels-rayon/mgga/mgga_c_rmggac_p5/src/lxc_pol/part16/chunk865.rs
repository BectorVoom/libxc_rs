//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 865/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk865(t39285: f64, t39295: f64, t39338: f64, t39405: f64, t39451: f64, t39528: f64, t39544: f64, t39591: f64, t2265: f64, t5026: f64, t39667: f64, t39678: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42906 = 0.39726959900411316772e-4_f64 * t39285;
    let t42909 = 0.39726959900411316772e-4_f64 * t39295;
    let t42928 = 0.60975299583150056624e-3_f64 * t39338;
    let t42954 = 0.39726959900411316772e-4_f64 * t39405;
    let t42970 = 0.3193131120497015617e0_f64 * t39451;
    let t43001 = 0.3193131120497015617e0_f64 * t39528;
    let t43008 = 0.47896966807455234256e0_f64 * t39544;
    let t43042 = 0.1489760996265424379e-3_f64 * t39591;
    let t43043 = t5026 * t2265;
    let t43096 = 0.10909864661698136692e0_f64 * t39667;
    let t43100 = 0.15965655602485078085e0_f64 * t39678;
    (t42906, t42909, t42928, t42954, t42970, t43001, t43008, t43042, t43043, t43096, t43100)
}
