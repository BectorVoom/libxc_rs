//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1192/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1192(t14731: f64, t3440: f64, t135: f64, t5045: f64, t1174: f64, t1222: f64, t4966: f64, t1215: f64, t1734: f64, t1089: f64, t475: f64, t607: f64) -> (f64, f64, f64, f64, f64) {
    let t15686 = t3440 * t14731;
    let t15689 = t135 * t5045;
    let t15691 = t1174 * t15689 / 432.0_f64;
    let t15699 = t4966 * t1222 / 2304.0_f64;
    let t15700 = t1734 * t1215;
    let t15701 = t475 * t1089;
    let t15702 = t15701 * t607;
    (t15686, t15691, t15699, t15700, t15702)
}
