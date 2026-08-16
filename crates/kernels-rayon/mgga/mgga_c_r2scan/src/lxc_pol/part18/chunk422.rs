//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 422/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk422(t166: f64, t2049: f64, t759: f64, t244: f64, t757: f64, t158: f64, t761: f64) -> (f64, f64, f64, f64, f64) {
    let t2050 = t166 * t2049;
    let t2052 = 0.285764e-1_f64 * t759 * t2050;
    let t2053 = t757 * t244;
    let t2054 = 1.0_f64 / t2053;
    let t2055 = t2054 * t158;
    let t2056 = t761 * t761;
    (t2050, t2052, t2054, t2055, t2056)
}
