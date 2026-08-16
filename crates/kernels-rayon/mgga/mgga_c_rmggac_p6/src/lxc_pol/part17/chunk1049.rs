//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1049/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1049(t2292: f64, t30221: f64, t36402: f64, t40679: f64, t40716: f64, t43677: f64, t47202: f64, t47207: f64, t47213: f64, t47215: f64, t47219: f64, t47223: f64, t47225: f64, t47229: f64, t47233: f64, t47235: f64, t47238: f64, t47242: f64, t5928: f64, t8933: f64) -> f64 {
    let t47244 = 0.76616279807936110914e-4_f64 * t47202 - 0.10215503974391481455e-3_f64 * t47207 + 0.10000709273223291967e0_f64 * t36402 - 0.82764499792523576607e-4_f64 * t40679 - t43677 + 0.79828278012425390428e-1_f64 * t5928 * t8933 - 0.2993560425465952141e-1_f64 * t47213 + 0.29795219925308487578e-4_f64 * t47215 + 0.79828278012425390428e-1_f64 * t30221 * t2292 + 0.17025839957319135759e-4_f64 * t47219 - 0.85129199786595678796e-5_f64 * t47223 + 0.23942587439980034662e-4_f64 * t47225 - 0.25538759935978703639e-4_f64 * t47229 + 0.25538759935978703639e-4_f64 * t47233 + 0.39914139006212695213e-1_f64 * t47235 - 0.13637330827122670864e-1_f64 * t47238 + 0.36366215538993788971e-1_f64 * t47242 + t40716;
    t47244
}
