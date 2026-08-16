//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 680/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk680(t587: f64, t972: f64, t151: f64, t177: f64, t2008: f64, t377: f64, t588: f64, t980: f64, t167: f64, t2035: f64) -> (f64, f64, f64, f64, f64) {
    let t7370 = t587 * t972;
    let t7372 = t151 * t7370 * t177;
    let t7373 = 0.11337795902333997111e-1_f64 * t7372;
    let t7375 = t377 * t2008 * t177;
    let t7376 = 0.40015750243531754508e-2_f64 * t7375;
    let t7378 = t980 * t588 * t177;
    let t7379 = 0.42874018118069736972e-3_f64 * t7378;
    let t7380 = t2035 * t167;
    (t7370, t7373, t7376, t7379, t7380)
}
