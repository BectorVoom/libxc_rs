//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1152/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1152(t2558: f64, t39002: f64, t9647: f64, t12311: f64, t2554: f64, t7064: f64, t123: f64, t1841: f64, t47182: f64, t734: f64, t1843: f64, t47188: f64) -> (f64, f64, f64, f64) {
    let t47594 = t9647 * t39002 * t2558;
    let t47597 = t7064 * t12311 * t2554;
    let t47602 = 0.85450291446024714263e-3_f64 * t1841 * t47182 * t123 * t734;
    let t47605 = 0.85450291446024714263e-3_f64 * t1841 * t1843 * t47188;
    (t47594, t47597, t47602, t47605)
}
