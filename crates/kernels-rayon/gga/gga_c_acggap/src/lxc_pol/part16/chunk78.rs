//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 78/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk78(t132: f64, t203: f64, t202: f64, t195: f64, t4: f64, t11: f64, t1: f64, t21: f64, t22: f64, t5: f64, t7: f64) -> (f64, f64, f64, f64, f64) {
    let t204 = t203 * t132;
    let t205 = t202 * t204;
    let t207 = t4 * t195;
    let t209 = f64::sqrt(t11);
    let t210 = t209 * t1;
    let t211 = t210 * t204;
    let t216 = t21 * t5 / t22 / t7;
    (t205, t207, t210, t211, t216)
}
