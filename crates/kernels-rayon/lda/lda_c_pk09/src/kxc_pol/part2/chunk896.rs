//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 896/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk896(t1106: f64, t9150: f64, t2143: f64, t4093: f64, t896: f64, t1101: f64, t2214: f64, t4085: f64, t4440: f64, t4478: f64, t4480: f64, t7776: f64, t8875: f64, t8879: f64, t8883: f64, t8887: f64, t8964: f64, t8966: f64, t8968: f64, t8970: f64, t8973: f64) -> f64 {
    let t9472 = t1106 * t9150;
    let t9480 = t896 * t4093 * t2143;
    let t9487 = -0.14975624337724558_f64 * t8875 - 0.14975624337724558_f64 * t8879 - 0.14975624337724558_f64 * t8883 - 0.01233429741534199_f64 * t8887 - t9472 / 6.0_f64 - t4478 - t4480 + 0.09983749558483038_f64 * t8964 - 0.016445729887122652_f64 * t8966 + 0.09983749558483038_f64 * t8968 + 0.016445729887122652_f64 * t8970 + 0.016445729887122652_f64 * t8973 - t4085 * t9480 / 36.0_f64 - t4440 * t2214 / 6.0_f64 - t1101 * t7776 / 6.0_f64;
    t9487
}
