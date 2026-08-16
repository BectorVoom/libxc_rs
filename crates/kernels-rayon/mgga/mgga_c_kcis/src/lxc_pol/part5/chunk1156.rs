//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1156/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1156(t1056: f64, t18574: f64, t10096: f64, t6272: f64, t331: f64, t6276: f64, t3160: f64, t1072: f64, t1064: f64, t18677: f64, t18672: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19473 = t1056 * t18574;
    let t19476 = t10096 * t6272;
    let t19478 = t331 * t6276;
    let t19480 = t3160 * t6276;
    let t19482 = t1072 * t6272;
    let t19488 = t1064 * t18677;
    let t19491 = t945 * t18672;
    (t19473, t19476, t19478, t19480, t19482, t19488, t19491)
}
