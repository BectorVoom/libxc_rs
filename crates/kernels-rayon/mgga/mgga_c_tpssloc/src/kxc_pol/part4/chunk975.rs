//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 975/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk975(t1213: f64, t15730: f64, t11789: f64, t1653: f64, t248: f64, t1227: f64, t15437: f64, t3505: f64, t3576: f64, t5064: f64, t13969: f64, t4988: f64) -> (f64, f64, f64, f64, f64) {
    let t15731 = t1213 * t15730;
    let t15734 = t248 * t11789 * t1653;
    let t15735 = t1227 * t15734;
    let t15737 = t15437 * t3505;
    let t15740 = t5064 * t3576;
    let t15743 = t13969 * t4988;
    (t15731, t15735, t15737, t15740, t15743)
}
