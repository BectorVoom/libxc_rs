//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1164/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1164(t6914: f64, t7737: f64, t1799: f64, t562: f64, t22705: f64, t7736: f64, t22704: f64, t6883: f64, t7741: f64, t7740: f64, t794: f64, t6897: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26406 = t6914 * t7737;
    let t26421 = t562 * t1799;
    let t26426 = t22705 * t7736;
    let t26427 = t22704 * t26426;
    let t26429 = t6883 * t7741;
    let t26436 = t794 * t7740;
    let t26437 = t6897 * t26436;
    (t26406, t26421, t26426, t26427, t26429, t26436, t26437)
}
