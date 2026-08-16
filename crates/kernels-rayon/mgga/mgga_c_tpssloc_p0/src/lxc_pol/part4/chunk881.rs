//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 881/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk881(t2617: f64, t2696: f64, t2693: f64, t809: f64, t597: f64, t61: f64, t241: f64, t244: f64, t248: f64, t238: f64, t154: f64, t9569: f64) -> (f64, f64, f64, f64, f64) {
    let t9993 = t2617 * t2696;
    let t10014 = t809 * t2693;
    let t10021 = 1.0_f64 / t61 / t597;
    let t10022 = t10021 * t241;
    let t10024 = t10022 * t244 * t248;
    let t10026 = 595.0_f64 / 10368.0_f64 * t238 * t10024;
    let t10027 = t9569 * t154;
    (t9993, t10014, t10022, t10026, t10027)
}
