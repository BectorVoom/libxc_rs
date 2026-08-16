//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 896/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk896<F: Float>(t1106: F, t9150: F, t2143: F, t4093: F, t896: F, t1101: F, t2214: F, t4085: F, t4440: F, t4478: F, t4480: F, t7776: F, t8875: F, t8879: F, t8883: F, t8887: F, t8964: F, t8966: F, t8968: F, t8970: F, t8973: F) -> F {
    let t9472 = t1106 * t9150;
    let t9480 = t896 * t4093 * t2143;
    let t9487 = -F::cast_from(0.14975624337724558_f64) * t8875 - F::cast_from(0.14975624337724558_f64) * t8879 - F::cast_from(0.14975624337724558_f64) * t8883 - F::cast_from(0.01233429741534199_f64) * t8887 - t9472 / F::cast_from(6.0_f64) - t4478 - t4480 + F::cast_from(0.09983749558483038_f64) * t8964 - F::cast_from(0.016445729887122652_f64) * t8966 + F::cast_from(0.09983749558483038_f64) * t8968 + F::cast_from(0.016445729887122652_f64) * t8970 + F::cast_from(0.016445729887122652_f64) * t8973 - t4085 * t9480 / F::cast_from(36.0_f64) - t4440 * t2214 / F::cast_from(6.0_f64) - t1101 * t7776 / F::cast_from(6.0_f64);
    t9487
}
