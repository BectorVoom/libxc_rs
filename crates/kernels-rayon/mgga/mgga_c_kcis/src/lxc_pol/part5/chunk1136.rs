//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1136/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1136(t1020: f64, t19123: f64, t3182: f64, t6491: f64, t1096: f64, t1092: f64, t6621: f64, t9429: f64, t9425: f64, t1646: f64, t4772: f64, t3203: f64) -> (f64, f64, f64, f64, f64) {
    let t19124 = t1020 * t19123;
    let t19126 = t3182 * t6491;
    let t19127 = t1096 * t19126;
    let t19128 = t1092 * t19127;
    let t19130 = t9429 * t6621;
    let t19132 = t9425 * t6621;
    let t19134 = t1646 * t4772;
    let t19135 = t3203 * t19134;
    (t19124, t19128, t19130, t19132, t19135)
}
