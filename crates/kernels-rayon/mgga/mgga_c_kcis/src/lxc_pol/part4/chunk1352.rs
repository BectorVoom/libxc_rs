//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1352/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1352(t16658: f64, t5909: f64, t5908: f64, t2047: f64, t4245: f64, t12568: f64, t5932: f64, t16653: f64, t4261: f64, t4260: f64, t492: f64, t6015: f64) -> (f64, f64, f64, f64, f64) {
    let t17402 = t5909 * t16658;
    let t17403 = t5908 * t17402;
    let t17405 = t4245 * t2047;
    let t17407 = t12568 * t5932;
    let t17409 = t4261 * t16653;
    let t17410 = t4260 * t17409;
    let t17412 = t6015 * t492;
    (t17403, t17405, t17407, t17410, t17412)
}
