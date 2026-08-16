//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 464/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk464(t2279: f64, t2281: f64, t2312: f64, t2315: f64, t2317: f64, t875: f64, t158: f64, t166: f64, t104: f64, t288: f64, t607: f64, t879: f64) -> (f64, f64, f64, f64, f64) {
    let t2320 = -0.571528e-1_f64 * t2279 + 0.285764e-1_f64 * t2281 + 0.285764e-1_f64 * t2312 * t875 - 0.285764e-1_f64 * t2315 * t2317;
    let t2321 = t2320 * t158;
    let t2322 = t2321 * t166;
    let t2323 = t104 * t288;
    let t2328 = t879 * t607;
    (t2320, t2321, t2322, t2323, t2328)
}
