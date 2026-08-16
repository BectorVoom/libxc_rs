//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 777/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk777(t3361: f64, t635: f64, t57: f64, t268: f64, t404: f64, t7021: f64, t159: f64, t3617: f64, t409: f64, t416: f64, t406: f64, t11335: f64, t281: f64, t414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12256 = 1.0_f64 / t3361 / t635;
    let t12267 = t3361 * t57;
    let t12268 = 1.0_f64 / t12267;
    let t12295 = t268 * t7021 * t404;
    let t12296 = 28.0_f64 / 27.0_f64 * t12295;
    let t12305 = t159 * t3617;
    let t12327 = 1.0_f64 / t409 / t416 / 4.0_f64;
    let t12331 = 1.0_f64/pow_3_2(t406);
    let t12349 = 0.93011851851851851854e0_f64 * t12295;
    let t12351 = t281 * t11335 * t414;
    (t12256, t12268, t12295, t12296, t12305, t12327, t12331, t12349, t12351)
}
