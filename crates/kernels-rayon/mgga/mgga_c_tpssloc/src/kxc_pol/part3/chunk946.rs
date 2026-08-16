//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 946/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk946(t11557: f64, t1174: f64, t135: f64, t3471: f64, t1184: f64, t4899: f64, t3242: f64, t460: f64, t2244: f64, t3448: f64, t3469: f64, t2250: f64, t3450: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11558 = t1174 * t11557;
    let t11560 = t135 * t3471;
    let t11561 = t1174 * t11560;
    let t11569 = t4899 * t1184;
    let t11570 = t460 * t3242;
    let t11571 = t11570 * t2244;
    let t11575 = t3448 * t3469;
    let t11579 = t3450 * t2250;
    (t11558, t11561, t11569, t11570, t11571, t11575, t11579)
}
