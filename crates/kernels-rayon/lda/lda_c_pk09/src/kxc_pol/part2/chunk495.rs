//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 495/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk495(t2730: f64, t497: f64, t489: f64, t1937: f64, t1939: f64, t2733: f64, t2736: f64, t337: f64, t430: f64, t1905: f64, t2149: f64, t309: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2769 = t497 * t2730;
    let t2772 = t489 * t2730;
    let t2777 = t1937 - 2.0_f64 * t2733 + t1939 + 2.0_f64 * t2736;
    let t2778 = t2777 * t337;
    let t2779 = t2778 * t430;
    let t2783 = t309 * t1905 * t2149;
    (t2769, t2772, t2777, t2778, t2779, t2783)
}
