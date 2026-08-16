//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 122/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk122(t28: f64, t14: f64, t167: f64, t2: f64, t4: f64, t7: f64) -> (f64, f64, f64, f64, f64) {
    let t382 = t28 * t28;
    let t383 = 1.0_f64 / t382;
    let t384 = t14 * t383;
    let t385 = t167 * t2;
    let t386 = t4 * t7;
    (t382, t383, t384, t385, t386)
}
