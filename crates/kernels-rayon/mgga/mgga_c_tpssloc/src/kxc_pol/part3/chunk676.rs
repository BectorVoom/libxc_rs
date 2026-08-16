//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 676/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk676(t1179: f64, t3431: f64, t1174: f64, t1186: f64, t135: f64, t1089: f64, t405: f64) -> (f64, f64, f64, f64, f64) {
    let t3432 = t3431 * t1179;
    let t3433 = t1174 * t3432;
    let t3435 = t135 * t1186;
    let t3436 = t1174 * t3435;
    let t3439 = 1.0_f64 / t405 / t1089;
    (t3432, t3433, t3435, t3436, t3439)
}
