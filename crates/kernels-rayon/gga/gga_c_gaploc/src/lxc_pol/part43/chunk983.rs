//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 983/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk983(t12311: f64, t2554: f64, t7064: f64, t123: f64, t1841: f64, t47182: f64, t734: f64, t1843: f64, t47188: f64, t47178: f64, t9647: f64, t39040: f64, t5539: f64) -> (f64, f64, f64, f64, f64) {
    let t47597 = t7064 * t12311 * t2554;
    let t47602 = 0.85450291446024714263e-3_f64 * t1841 * t47182 * t123 * t734;
    let t47605 = 0.85450291446024714263e-3_f64 * t1841 * t1843 * t47188;
    let t47607 = t9647 * t1843 * t47178;
    let t47610 = t9647 * t5539 * t39040;
    (t47597, t47602, t47605, t47607, t47610)
}
