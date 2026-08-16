//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2196/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2196(t86752: f64, t86801: f64, t87952: f64, t88001: f64, t25608: f64, t381: f64, t25428: f64, t6712: f64, t13797: f64, t1926: f64, t221: f64, t10216: f64, t387: f64) -> (f64, f64, f64, f64, f64) {
    let t88003 = t86752 + t86801 + t87952 + t88001;
    let t88004 = t25608 * t381;
    let t88016 = t6712 * t25428;
    let t88022 = t1926 * t221 * t13797;
    let t88023 = t387 * t10216;
    (t88003, t88004, t88016, t88022, t88023)
}
