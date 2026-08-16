//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 511/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk511(t2089: f64, t827: f64, t22: f64, t3118: f64, t2097: f64, t45: f64, t2105: f64, t3696: f64, t2059: f64, t443: f64, t3859: f64, t212: f64, t23: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5736 = t827 * t2089;
    let t5744 = t22 * t3118;
    let t5765 = t45 * t2097;
    let t5770 = t3696 * t2105;
    let t5802 = t443 * t2059;
    let t5804 = t3859 * t2059;
    let t5814 = 1.0_f64 / t23 / t212;
    (t5736, t5744, t5765, t5770, t5802, t5804, t5814)
}
