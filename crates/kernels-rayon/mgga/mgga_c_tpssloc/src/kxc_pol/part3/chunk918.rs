//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 918/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk918(t1015: f64, t10515: f64, t1012: f64, t2928: f64, t320: f64, t10294: f64, t268: f64, t271: f64, t6546: f64, t2394: f64, t885: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10516 = t1015 * t10515;
    let t10517 = t1012 * t10516;
    let t10523 = 1.0_f64 / t2928 / t320;
    let t10542 = 0.36793333333333333333e0_f64 * t10294;
    let t10544 = t268 * t6546 * t271;
    let t10545 = 0.93932222222222222223e0_f64 * t10544;
    let t10556 = t2394 * t885;
    (t10517, t10523, t10542, t10544, t10545, t10556)
}
