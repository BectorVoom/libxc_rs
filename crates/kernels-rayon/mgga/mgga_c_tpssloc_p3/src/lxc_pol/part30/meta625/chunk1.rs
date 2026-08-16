//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2027/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2027(t22893: f64, t23164: f64, t25306: f64, t7524: f64, t81612: f64, t81613: f64, t4250: f64, t81749: f64, t23145: f64, t4166: f64, t22690: f64, t234: f64) -> (f64, f64, f64, f64, f64) {
    let t87165 = t23164 * t22893 * t25306;
    let t87166 = 0.16449340668482264365e-1_f64 * t87165;
    let t87177 = t81612 * t81613 * t7524;
    let t87197 = t81749 * t4250;
    let t87198 = 7.0_f64 / 288.0_f64 * t87197;
    let t87199 = t4166 * t23145;
    let t87202 = t22690 * t234;
    (t87166, t87177, t87198, t87199, t87202)
}
