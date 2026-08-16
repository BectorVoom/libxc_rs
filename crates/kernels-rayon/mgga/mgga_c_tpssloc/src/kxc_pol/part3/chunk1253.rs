//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1253/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1253(t4072: f64, t671: f64, t1458: f64, t2363: f64, t12521: f64, t12524: f64, t12813: f64, t1401: f64, t16506: f64, t16521: f64, t16524: f64, t16535: f64, t2319: f64, t3938: f64, t3941: f64, t5371: f64, t5376: f64, t577: f64) -> f64 {
    let t16538 = t4072 * t671;
    let t16541 = t1458 * t2363;
    let t16546 = 0.45e1_f64 * t16506 * t577 + 27.0_f64 * t16521 * t671 + 27.0_f64 * t16524 * t2319 + 0.135e2_f64 * t5371 * t2363 + 0.135e2_f64 * t12521 * t1458 + 54.0_f64 * t12524 * t5376 + 27.0_f64 * t3938 * t4072 + 27.0_f64 * t16535 * t1458 + 54.0_f64 * t3941 * t16538 + 27.0_f64 * t3941 * t16541 + 0.135e2_f64 * t1401 * t12813;
    t16546
}
