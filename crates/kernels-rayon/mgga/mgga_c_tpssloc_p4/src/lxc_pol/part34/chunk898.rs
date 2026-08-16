//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 898/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk898(t21222: f64, t21237: f64, t951: f64, t10632: f64, t21089: f64, t13727: f64, t5695: f64, t1556: f64, t5694: f64, t913: f64, t2842: f64, t10756: f64, t10771: f64, t10811: f64, t10828: f64, t14263: f64, t14271: f64, t14337: f64, t1569: f64, t1581: f64, t17355: f64, t17428: f64, t21115: f64, t21195: f64, t21198: f64, t21207: f64, t2930: f64, t4411: f64, t4449: f64, t5759: f64, t5762: f64, t5775: f64, t5791: f64, t5794: f64, t924: f64, t943: f64) -> (f64, f64, f64, f64, f64) {
    let t21238 = t21222 + t21237;
    let t21239 = t21238 * t951;
    let t21242 = t21089 * t10632;
    let t21247 = t21089 * t951;
    let t21251 = 6.0_f64 * t13727 * t5695;
    let t21252 = t5694 * t1556;
    let t21253 = t21252 * t913;
    let t21255 = 6.0_f64 * t2842 * t21253;
    let t21256 = 3.0_f64 * t17428 * t1569 + 3.0_f64 * t4411 * t5759 + 0.96491876992155210402e2_f64 * t14271 * t5762 - 0.19298375398431042081e3_f64 * t10771 * t21115 + 1.0_f64 * t924 * t21195 + 0.2069040516770936012e4_f64 * t10811 * t21198 + 0.17544670867903938621e1_f64 * t17355 * t1581 + 0.17544670867903938621e1_f64 * t4449 * t5791 + 0.51947577317044391276e2_f64 * t14337 * t5794 - 0.10389515463408878255e3_f64 * t10828 * t21207 + 0.5848223622634646207e0_f64 * t943 * t21239 + 0.10254018858216406658e4_f64 * t10756 * t21242 - 0.35089341735807877242e1_f64 * t14263 * t5775 + 0.35089341735807877242e1_f64 * t2930 * t21247 + t21251 - t21255;
    (t21238, t21251, t21252, t21255, t21256)
}
