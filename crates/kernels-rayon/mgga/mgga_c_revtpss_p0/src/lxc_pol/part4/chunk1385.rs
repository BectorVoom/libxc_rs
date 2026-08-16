//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1385/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1385(t17539: f64, t5296: f64, t1042: f64, t3172: f64, t5286: f64, t1247: f64, t3707: f64, t5292: f64, t12268: f64, t3617: f64, t15936: f64, t3708: f64, t5265: f64) -> (f64, f64, f64, f64, f64) {
    let t17540 = t5296 * t17539;
    let t17541 = t1042 * t17540;
    let t17544 = t3172 * t5286;
    let t17546 = 0.28582678745379824648e-3_f64 * t1247 * t17544;
    let t17547 = t3707 * t5292;
    let t17550 = t3617 * t12268;
    let t17551 = t17550 * t15936;
    let t17552 = t1042 * t17551;
    let t17556 = 0.28582678745379824648e-3_f64 * t3708 * t5265;
    (t17541, t17546, t17547, t17552, t17556)
}
