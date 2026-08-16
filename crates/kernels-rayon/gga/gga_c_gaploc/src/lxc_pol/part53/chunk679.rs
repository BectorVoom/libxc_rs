//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 679/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk679(t712: f64, t3221: f64, t12390: f64, t5337: f64, t5340: f64, t5345: f64, t5348: f64, t1692: f64, t3222: f64, t12380: f64, t713: f64, t928: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t12557 = pi * t712;
    let t12558 = t3221 * t12557;
    let t12561 = t12390 * t5337 * t5340;
    let t12564 = t5345 * t12390 * t5348;
    let t12566 = t1692 * t3222;
    let t12568 = t713 * t12380;
    let t12569 = t12568 * t928;
    (t12557, t12558, t12561, t12564, t12566, t12568, t12569)
}
