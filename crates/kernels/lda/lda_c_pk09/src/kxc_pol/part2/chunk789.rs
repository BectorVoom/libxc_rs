//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 789/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk789<F: Float>(t1106: F, t9150: F, t2143: F, t4093: F, t896: F, t1101: F, t2214: F, t4085: F, t4440: F, t4478: F, t4480: F, t7776: F, t8875: F, t8879: F, t8883: F, t8887: F, t8964: F, t8966: F, t8968: F, t8970: F, t8973: F) -> (F,) {
    let t9472 = t1106 * t9150;
    let t9480 = t896 * t4093 * t2143;
    let t9487 = -0.14975624337724558 * t8875 - 0.14975624337724558 * t8879 - 0.14975624337724558 * t8883 - 0.01233429741534199 * t8887 - t9472 / 6.0 - t4478 - t4480 + 0.09983749558483038 * t8964 - 0.016445729887122652 * t8966 + 0.09983749558483038 * t8968 + 0.016445729887122652 * t8970 + 0.016445729887122652 * t8973 - t4085 * t9480 / 36.0 - t4440 * t2214 / 6.0 - t1101 * t7776 / 6.0;
    (t9487,)
}
