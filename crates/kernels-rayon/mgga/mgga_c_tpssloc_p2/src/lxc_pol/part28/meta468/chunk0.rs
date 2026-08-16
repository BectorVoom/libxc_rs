//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1677/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1677(t25154: f64, t25155: f64, t253: f64, t254: f64, t1484: f64, t857: f64, t865: f64, t23270: f64, t22986: f64, t23204: f64, t7488: f64, t6562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25156 = t25154 * t25155;
    let t25168 = t253 * t254;
    let t25191 = t857 * t1484;
    let t25192 = t25191 * t865;
    let t25193 = t23270 * t25192;
    let t25194 = t22986 * t25193;
    let t25205 = t23204 * t7488;
    let t25206 = t6562 * t25205;
    (t25156, t25168, t25191, t25192, t25193, t25194, t25205, t25206)
}
