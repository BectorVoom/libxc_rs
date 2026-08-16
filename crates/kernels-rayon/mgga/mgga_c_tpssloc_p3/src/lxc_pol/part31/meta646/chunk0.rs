//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1918/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1918(t16662: f64, t6552: f64, t6553: f64, t6554: f64, t23164: f64, t23204: f64, t28276: f64, t16968: f64, t87052: f64, t87053: f64, t16887: f64, t87057: f64) -> (f64, f64, f64, f64) {
    let t98319 = t6552 * t6553 * t6554 * t16662;
    let t98322 = t23164 * t23204 * t28276;
    let t98325 = t87052 * t87053 * t16968;
    let t98328 = t87057 * t87053 * t16887;
    (t98319, t98322, t98325, t98328)
}
