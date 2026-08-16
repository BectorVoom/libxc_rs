//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1228/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1228(t12266: f64, t5677: f64, t1464: f64, t3734: f64, t5881: f64, t3801: f64, t5632: f64, t1395: f64, t1394: f64, t2001: f64, t4136: f64, t4135: f64) -> (f64, f64, f64, f64, f64) {
    let t15850 = t12266 * t5677;
    let t15851 = t1464 * t15850;
    let t15853 = t3734 * t5881;
    let t15854 = t1464 * t15853;
    let t15856 = t5632 * t3801;
    let t15857 = t1395 * t15856;
    let t15858 = t1394 * t15857;
    let t15860 = t2001 * t4136;
    let t15861 = t4135 * t15860;
    (t15851, t15854, t15858, t15860, t15861)
}
