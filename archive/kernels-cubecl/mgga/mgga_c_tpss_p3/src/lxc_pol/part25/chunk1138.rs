//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1138/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1138<F: Float>(t15248: F, t15251: F, t15292: F, t15294: F, t15296: F, t15299: F, t15301: F, t15304: F, t15307: F, t15309: F, t15312: F, t11938: F, t12060: F, t15264: F, t15268: F, t15273: F, t15277: F, t15283: F, t15288: F, t15334: F, t15339: F, t15342: F) -> (F, F) {
    let t15669 = -F::cast_from(0.69463333333333333333e-1_f64) * t15248 + F::cast_from(0.516475e0_f64) * t15251 + F::cast_from(0.3529725e1_f64) * t15292 + F::cast_from(0.6311625e0_f64) * t15294 + F::cast_from(0.23154444444444444445e-1_f64) * t15296 - F::cast_from(0.157790625e0_f64) * t15299 + F::cast_from(0.6311625e0_f64) * t15301 + F::cast_from(0.31558125e0_f64) * t15304 + F::cast_from(0.264729375e1_f64) * t15307 - F::cast_from(0.3529725e1_f64) * t15309 - F::cast_from(0.17648625e1_f64) * t15312;
    let t15690 = -F::cast_from(0.20839e0_f64) * t15334 + F::cast_from(0.45908888888888888888e0_f64) * t11938 - t12060 - F::cast_from(0.34431666666666666667e0_f64) * t15283 + F::cast_from(0.46308888888888888889e-1_f64) * t15339 - F::cast_from(0.34731666666666666667e-1_f64) * t15342 - F::cast_from(0.68863333333333333334e0_f64) * t15268 - F::cast_from(0.20659e1_f64) * t15264 + F::cast_from(0.20659e1_f64) * t15277 + F::cast_from(0.309885e1_f64) * t15273 + F::cast_from(0.103295e1_f64) * t15288;
    (t15669, t15690)
}
