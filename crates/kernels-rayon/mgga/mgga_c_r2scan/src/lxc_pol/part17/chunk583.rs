//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 583/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk583(t106: f64, t3232: f64, t797: f64, t97: f64, t1356: f64, t1360: f64, t1387: f64, t1413: f64, t2322: f64, t2460: f64, t2891: f64, t2895: f64, t2896: f64, t2897: f64, t2997: f64, t2998: f64, t3019: f64, t3128: f64, t3162: f64, t3165: f64, t372: f64, t881: f64) -> f64 {
    let t3235 = t97 * t106 * t3232 * t797;
    let t3243 = 2.0_f64 * t2460 - t2891 + t1356 + t1360 - t2895 - t2896 + t2897 + t2997 + t2998 + t1387 + t1413 + t2322 - 0.2363e1_f64 * t881 * t3162 - 0.4726e1_f64 * t881 * t3165 + t372 * t3128 - t3019 - t3235;
    t3243
}
