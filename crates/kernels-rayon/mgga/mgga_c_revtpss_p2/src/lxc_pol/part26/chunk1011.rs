//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1011/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1011(t10356: f64, t13020: f64, t1012: f64, t3367: f64, t404: f64, t12256: f64, t1204: f64, t3140: f64, t3599: f64, t11239: f64, t460: f64, t1242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13021 = t13020 * t10356;
    let t13022 = t1012 * t13021;
    let t13026 = 1.0_f64 / t404 / t3367;
    let t13027 = t13026 * t12256;
    let t13028 = t13027 * t10356;
    let t13029 = t1012 * t13028;
    let t13032 = t1204 * t3140;
    let t13033 = t13032 * t3599;
    let t13036 = t460 * t11239;
    let t13037 = t1242 * t1242;
    (t13022, t13029, t13032, t13033, t13036, t13037)
}
