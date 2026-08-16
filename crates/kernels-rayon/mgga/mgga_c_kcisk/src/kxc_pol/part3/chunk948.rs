//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 948/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk948(t3897: f64, t970: f64, t1186: f64, t12952: f64, t12957: f64, t1383: f64, t1398: f64, t1375: f64, t14014: f64, t14016: f64, t14019: f64, t14022: f64, t14025: f64, t14027: f64, t14029: f64, t158: f64, t165: f64, t173: f64) -> f64 {
    let t14031 = t970 * t3897;
    let t14033 = t1186 * t12952;
    let t14036 = t1383 * t12957;
    let t14039 = t1398 * t12957;
    let t14042 = t1375 * t12957;
    let t14045 = 0.26416666666666666666e-2_f64 * t14014 + 0.7925e-3_f64 * t165 * t14016 - 0.17611111111111111111e-3_f64 * t165 * t14019 - 0.7026e-2_f64 * t158 * t14022 + 0.79249999999999999999e-2_f64 * t14025 + 0.10566666666666666666e-1_f64 * t14027 + 0.117630625e-3_f64 * t14029 + 0.70578375e-4_f64 * t14031 + 0.317e-2_f64 * t165 * t14033 + 0.4755e-2_f64 * t165 * t14036 + 0.30247875e-4_f64 * t173 * t14039 - 0.21078e-1_f64 * t158 * t14042;
    t14045
}
