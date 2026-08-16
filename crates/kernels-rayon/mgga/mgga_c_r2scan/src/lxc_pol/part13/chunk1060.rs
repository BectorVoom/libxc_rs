//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1060/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1060(t37541: f64, t10913: f64, t1561: f64, t10954: f64, t10958: f64, t3446: f64, t10962: f64, t10949: f64, t2312: f64, t3447: f64, t3438: f64, t6868: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37542 = 0.24390119833260022651e-2_f64 * t37541;
    let t37543 = t1561 * t10913;
    let t37556 = t3446 * t10954 * t10958;
    let t37560 = t3446 * t10954 * t10962;
    let t37561 = 0.12195059916630011326e-2_f64 * t37560;
    let t37564 = t3446 * t3447 * t10949 * t2312;
    let t37568 = t3446 * t3447 * t3438 * t6868;
    (t37542, t37543, t37556, t37561, t37564, t37568)
}
