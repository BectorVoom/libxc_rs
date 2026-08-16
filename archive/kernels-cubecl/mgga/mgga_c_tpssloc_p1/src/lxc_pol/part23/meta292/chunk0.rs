//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1010/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1010<F: Float>(t21238: F, t951: F, t10632: F, t21089: F, t13727: F, t5695: F, t1556: F, t5694: F, t913: F, t2842: F, t10756: F, t10771: F, t10811: F, t10828: F, t14263: F, t14271: F, t14337: F, t1569: F, t1581: F, t17355: F, t17428: F, t21115: F, t21195: F, t21198: F, t21207: F, t2930: F, t4411: F, t4449: F, t5759: F, t5762: F, t5775: F, t5791: F, t5794: F, t924: F, t943: F) -> (F, F, F, F, F, F, F, F) {
    let t21239 = t21238 * t951;
    let t21242 = t21089 * t10632;
    let t21247 = t21089 * t951;
    let t21251 = F::cast_from(6.0_f64) * t13727 * t5695;
    let t21252 = t5694 * t1556;
    let t21253 = t21252 * t913;
    let t21255 = F::cast_from(6.0_f64) * t2842 * t21253;
    let t21256 = F::cast_from(3.0_f64) * t17428 * t1569 + F::cast_from(3.0_f64) * t4411 * t5759 + F::cast_from(0.96491876992155210402e2_f64) * t14271 * t5762 - F::cast_from(0.19298375398431042081e3_f64) * t10771 * t21115 + F::cast_from(1.0_f64) * t924 * t21195 + F::cast_from(0.2069040516770936012e4_f64) * t10811 * t21198 + F::cast_from(0.17544670867903938621e1_f64) * t17355 * t1581 + F::cast_from(0.17544670867903938621e1_f64) * t4449 * t5791 + F::cast_from(0.51947577317044391276e2_f64) * t14337 * t5794 - F::cast_from(0.10389515463408878255e3_f64) * t10828 * t21207 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t21239 + F::cast_from(0.10254018858216406658e4_f64) * t10756 * t21242 - F::cast_from(0.35089341735807877242e1_f64) * t14263 * t5775 + F::cast_from(0.35089341735807877242e1_f64) * t2930 * t21247 + t21251 - t21255;
    (t21239, t21242, t21247, t21251, t21252, t21253, t21255, t21256)
}
