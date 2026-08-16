//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 905/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk905(t3242: f64, t460: f64, t3247: f64, t1176: f64, t134: f64, t1184: f64, t1239: f64, t68: f64, t1203: f64, t3540: f64, t2393: f64, t374: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11570 = t460 * t3242;
    let t11583 = t460 * t3247;
    let t11588 = t134 * t1176;
    let t11589 = t11588 * t1184;
    let t11604 = t1239 * t1239;
    let t11605 = 1.0_f64 / t11604;
    let t11606 = t68 * t11605;
    let t11644 = t1203 * t3540;
    let t11647 = t374 * t2393 * t486;
    (t11570, t11583, t11588, t11589, t11606, t11644, t11647)
}
