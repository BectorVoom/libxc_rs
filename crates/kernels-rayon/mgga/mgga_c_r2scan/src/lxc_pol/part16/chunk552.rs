//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 552/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk552(t3034: f64, t86: f64, t88: f64, t41: f64, t1387: f64, t1413: f64, t1418: f64, t1421: f64, t2896: f64, t2897: f64, t2997: f64, t2998: f64, t3020: f64) -> (f64, f64, f64, f64) {
    let t3035 = t3034 * t86;
    let t3036 = 0.19751673498613801407e-1_f64 * t3035;
    let t3037 = t3034 * t88;
    let t3038 = t41 * t3037;
    let t3039 = -t3020 + t2896 - t2897 - t2998 + t3036 + t3038 - t2997 - t1387 - t1413 + t1418 + t1421;
    (t3036, t3037, t3038, t3039)
}
