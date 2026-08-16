//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1166/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1166(t19164: f64, t19207: f64, t1241: f64, t1235: f64, t6150: f64, t1760: f64, t5088: f64, t3598: f64, t1251: f64, t6267: f64, t6243: f64, t11606: f64) -> (f64, f64, f64, f64, f64) {
    let t19208 = t19164 + t19207;
    let t19209 = t1241 * t19208;
    let t19211 = t6150 * t1235;
    let t19213 = t1760 * t5088;
    let t19214 = t3598 * t19213;
    let t19219 = t6267 * t1251;
    let t19220 = t3598 * t19219;
    let t19225 = t6243 * t1251;
    let t19226 = t11606 * t19225;
    (t19209, t19211, t19214, t19220, t19226)
}
