//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 759/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk759(t2049: f64, t864: f64, t2287: f64, t244: f64, t6007: f64, t2279: f64, t2292: f64, t2288: f64, t357: f64, t761: f64, t366: f64, t2281: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6813 = t864 * t2049;
    let t6817 = 1.0_f64 / t2287 / t244;
    let t6818 = t6817 * t6007;
    let t6821 = t2279 * t2292;
    let t6826 = t2288 * t2049;
    let t6827 = t761 * t357;
    let t6828 = t6827 * t366;
    let t6831 = t2281 * t2292;
    (t6813, t6818, t6821, t6826, t6828, t6831)
}
