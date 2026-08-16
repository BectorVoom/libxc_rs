//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1941/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1941(t1089: f64, t29759: f64, t1972: f64, t6317: f64, t1675: f64, t25538: f64, t27448: f64, t27460: f64, t27462: f64, t27471: f64, t27489: f64, t375: f64, t6285: f64, t6289: f64, t6293: f64, t6323: f64, t6327: f64, t7111: f64, t7132: f64) -> (f64, f64, f64) {
    let t29760 = t29759 * t1089;
    let t29779 = t6317 * t1972;
    let t29782 = 0.57165357490759649296e-3_f64 * t27448 + t27460 / 432.0_f64 + 0.57165357490759649296e-3_f64 * t27462 - 0.57165357490759649296e-3_f64 * t27471 - t7111 * t6285 / 144.0_f64 + t7111 * t6289 / 288.0_f64 + t7111 * t6293 / 216.0_f64 + 0.28582678745379824648e-3_f64 * t7132 * t6323 + 0.47637797908966374413e-3_f64 * t7132 * t6327 + 0.57165357490759649296e-3_f64 * t27489 * t1675 - t25538 + 0.42874018118069736972e-3_f64 * t29779 * t375;
    (t29760, t29779, t29782)
}
