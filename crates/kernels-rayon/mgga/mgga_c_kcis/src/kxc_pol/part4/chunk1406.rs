//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1406/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1406(t17331: f64, t17335: f64, t17337: f64, t17339: f64, t17342: f64, t17344: f64, t17347: f64, t17350: f64, t17353: f64, t17355: f64, t17358: f64, t17360: f64, t17362: f64, t17364: f64, t17366: f64, t17368: f64, t17371: f64, t17374: f64) -> f64 {
    let t18292 = -0.101171875e-1_f64 * t17331 - 0.53958333333333333333e-1_f64 * t17335 + 0.625e-1_f64 * t17337 - 0.33333333333333333334e0_f64 * t17339 - 0.44965277777777777777e-2_f64 * t17342 - 0.1875e0_f64 * t17344 - 0.41666666666666666666e-1_f64 * t17347 + 0.44965277777777777777e-2_f64 * t17350 + 0.10791666666666666667e0_f64 * t17353 - 0.9375e-1_f64 * t17355 + 0.375e0_f64 * t17358 - 0.9375e-1_f64 * t17360 + 0.53958333333333333333e-1_f64 * t17362 - 0.125e0_f64 * t17364 + 0.26979166666666666666e-1_f64 * t17366 - 0.44965277777777777777e-2_f64 * t17368 + 0.20833333333333333333e-1_f64 * t17371 - 0.4046875e-1_f64 * t17374;
    t18292
}
