//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1187/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1187(t11697: f64, t4953: f64, t3577: f64, t12648: f64, t4972: f64, t4582: f64, t1229: f64, t3242: f64, t14165: f64, t3493: f64, t3508: f64, t4977: f64) -> (f64, f64, f64, f64) {
    let t15608 = t11697 * t4953;
    let t15610 = t3577 * t15608 / 3456.0_f64;
    let t15611 = t4972 * t12648;
    let t15612 = t4582 * t15611;
    let t15615 = t1229 * t3242;
    let t15616 = t15615 * t14165;
    let t15617 = t4582 * t15616;
    let t15620 = t3508 * t3493;
    let t15621 = t4977 * t15620;
    (t15610, t15612, t15617, t15621)
}
