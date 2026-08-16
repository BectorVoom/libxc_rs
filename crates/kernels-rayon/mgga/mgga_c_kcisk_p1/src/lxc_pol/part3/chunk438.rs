//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 438/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk438(t1136: f64, t1139: f64, t1138: f64, t288: f64, t285: f64, t1147: f64, t3234: f64, t3237: f64, t3239: f64, t3243: f64, t3246: f64, t3249: f64, t3251: f64, t3254: f64, t3256: f64, t3258: f64, t3261: f64, t3264: f64, t3267: f64, t3272: f64) -> (f64, f64, f64, f64, f64) {
    let t3437 = t1136 * t1139;
    let t3441 = 1.0_f64 / t1138 / t288;
    let t3442 = t285 * t3441;
    let t3443 = t1147 * t1147;
    let t3460 = 0.1875e0_f64 * t3234 - 0.375e0_f64 * t3237 - 0.75e0_f64 * t3239 + 0.375e0_f64 * t3243 + 0.75e0_f64 * t3246 - 0.1875e0_f64 * t3249 + 0.1125e1_f64 * t3251 - 0.4046875e-1_f64 * t3254 + 0.809375e-1_f64 * t3256 + 0.32375e0_f64 * t3258 - 0.809375e-1_f64 * t3261 - 0.32375e0_f64 * t3264 + 0.4046875e-1_f64 * t3267 - 0.809375e0_f64 * t3272;
    (t3437, t3441, t3442, t3443, t3460)
}
