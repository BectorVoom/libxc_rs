//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 241/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk241(t227: f64, t1060: f64, t229: f64, t1059: f64, t44: f64, t247: f64, t242: f64, t819: f64, t821: f64, t825: f64, t827: f64, t250: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t1063 = piecewise3(t228, 0.0_f64, 4.0_f64 / 3.0_f64 * t229 * t1060);
    let t1065 = (t1059 + t1063) * t44;
    let t1070 = t247 * t247;
    let t1071 = 1.0_f64 / t1070;
    let t1072 = t242 * t1071;
    let t1077 = -0.1176575e1_f64 * t819 - 0.516475e0_f64 * t821 - 0.2103875e0_f64 * t825 - 0.104195e0_f64 * t827;
    let t1078 = 1.0_f64 / t250;
    (t1065, t1070, t1071, t1072, t1077, t1078)
}
