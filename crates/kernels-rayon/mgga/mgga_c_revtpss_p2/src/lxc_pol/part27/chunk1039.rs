//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1039/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1039(t1209: f64, t3727: f64, t460: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64) -> (f64, f64, f64) {
    let t12666 = t1209 * t3727;
    let t12673 = t460 * t3727;
    let t12678 = 0.25925925925925925926e-1_f64 * t12295;
    let t12689 = -t12678 + 0.11111111111111111111e-1_f64 * t12297 + 0.55555555555555555555e-2_f64 * t12299 - 0.16666666666666666667e-1_f64 * t12301 - 0.83333333333333333334e-2_f64 * t12303 + 0.92592592592592592592e-2_f64 * t12307 - 0.33333333333333333333e-1_f64 * t12310 - 0.16666666666666666666e-1_f64 * t12292 + 0.50000000000000000001e-1_f64 * t12314 + 0.50000000000000000001e-1_f64 * t12317 + 0.83333333333333333333e-2_f64 * t12320;
    (t12666, t12673, t12689)
}
