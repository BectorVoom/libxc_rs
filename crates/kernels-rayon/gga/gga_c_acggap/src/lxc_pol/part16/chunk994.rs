//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 994/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk994(t35190: f64, t1181: f64, t20311: f64, t7351: f64, t7426: f64, t1165: f64, t21118: f64, t8600: f64, t7637: f64, t8555: f64, t1967: f64, t8549: f64) -> (f64, f64, f64, f64, f64) {
    let t35191 = 0.47172138434406228102e-2_f64 * t35190;
    let t35194 = t7426 * t1181 * t7351 * t20311;
    let t35195 = 0.18868855373762491241e-2_f64 * t35194;
    let t35198 = t7426 * t1165 * t8600 * t21118;
    let t35199 = 0.37737710747524982482e-2_f64 * t35198;
    let t35204 = t7637 * t8555;
    let t35210 = t1967 * t8549;
    (t35191, t35195, t35199, t35204, t35210)
}
